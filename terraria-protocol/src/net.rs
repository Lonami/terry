//! Basic, threaded network implementation.
use crate::serde::{PacketBody, SliceCursor};
use crate::{packets, Packet};
use log::trace;
use std::io::{self, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc;
use std::thread;

const PROTOCOL_VERSION: &str = "Terraria238";

// TODO don't use constants for this
const PLAYER_UUID: &str = "01032c81-623f-4435-85e5-e0ec816b09ca"; // random

const READ_MESSAGE_BUFFER: usize = 16;

pub struct Terraria {
    stream: TcpStream,
    out_buffer: Vec<u8>,
    _reader_thread: thread::JoinHandle<io::Result<()>>,
    packet_rx: mpsc::Receiver<Packet>,
}

fn as_hex(buf: &[u8]) -> String {
    const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut bytes = Vec::with_capacity(buf.len() * 2);
    buf.into_iter().for_each(|b| {
        bytes.push(HEX_DIGITS[(b >> 4) as usize]);
        bytes.push(HEX_DIGITS[(b & 15) as usize]);
    });
    // SAFETY: we're only pushing ASCII characters (i.e. valid UTF-8)
    unsafe { String::from_utf8_unchecked(bytes) }
}

fn reader_worker(
    mut reader: BufReader<TcpStream>,
    sender: mpsc::SyncSender<Packet>,
) -> io::Result<()> {
    let mut lenbuf = [0u8; 2];
    let mut packet = Vec::new();

    loop {
        reader.read_exact(&mut lenbuf)?;
        let mut cursor = SliceCursor::new(&mut lenbuf);
        let len = cursor.read::<u16>() as usize - 2;
        cursor.finish();

        packet.reserve(len);
        while packet.len() < len {
            packet.push(0);
        }
        reader.read_exact(&mut packet[..len])?;

        trace!(
            "< {} : {}{}",
            packet[0],
            as_hex(&lenbuf),
            as_hex(&packet[..len])
        );
        if sender.send(Packet::from_slice(&mut packet[..len])).is_err() {
            break Ok(());
        }
    }
}

impl Terraria {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        // connection
        let stream = TcpStream::connect(addr)?;
        let reader = BufReader::new(stream.try_clone()?);
        let (packet_tx, packet_rx) = mpsc::sync_channel(READ_MESSAGE_BUFFER);
        let _reader_thread = thread::Builder::new()
            .name("reader thread".to_string())
            .spawn(move || reader_worker(reader, packet_tx))?;
        let mut this = Self {
            stream,
            out_buffer: vec![0; 1024],
            _reader_thread,
            packet_rx,
        };

        // handshake
        this.send_packet(&packets::Connect {
            version: PROTOCOL_VERSION.to_string(),
        })?;

        // TODO this needs to be customizable
        this.send_packet(&packets::PlayerInfo::default())?;

        this.send_packet(&packets::ClientUuid {
            uuid4: PLAYER_UUID.to_string(),
        })?;

        // TODO rename to Health?
        this.send_packet(&packets::PlayerHP {
            player_id: 0,
            hp: 100,
            max_hp: 100,
        })?;

        this.send_packet(&packets::PlayerMana {
            player_id: 0,
            mana: 200,
            max_mana: 200,
        })?;

        // TODO bad name
        this.send_packet(&packets::UpdatePlayerBuff {
            player_id: 0,
            buffs: [0u16; 22],
        })?;

        for i in 0..260 {
            this.send_packet(&packets::PlayerInventorySlot {
                player_id: 0,
                slot_id: i,
                stack: 0,
                prefix: 0,
                item_netid: 0,
            })?;
        }

        this.send_packet(&packets::RequestWorldData {})?;

        this.send_packet(&packets::RequestEssentialTiles {
            spawn_x: -1,
            spawn_y: -1,
        })?;

        this.send_packet(&packets::SpawnPlayer {
            player_id: 0,
            spawn_x: -1,
            spawn_y: -1,
            respawn_time_remaining: 0,
            player_spawn_context: packets::SpawnContext::SpawningIntoWorld,
        })?;

        Ok(this)
    }

    pub fn send_packet<P: PacketBody>(&mut self, packet: &P) -> io::Result<()> {
        let mut cursor = SliceCursor::new(self.out_buffer.as_mut_slice());
        packet.serialize(&mut cursor);
        let (slice, pos) = cursor.finish();
        self.stream.write_all(&slice[..pos])?;
        self.stream.flush()?;
        trace!("> {} : {}", P::TAG, as_hex(&slice[..pos]));
        Ok(())
    }

    pub fn recv_packet(&mut self) -> Result<Packet, mpsc::RecvError> {
        self.packet_rx.recv()
    }

    pub fn try_recv_packet(&mut self) -> Result<Packet, mpsc::TryRecvError> {
        self.packet_rx.try_recv()
    }
}
