use crate::packets::{self, Packet, PacketBody};
use crate::SliceCursor;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

const PROTOCOL_VERSION: &str = "Terraria228";

// TODO don't use constants for this
//const PLAYER_UUID: &str = "01032c81-623f-4435-85e5-e0ec816b09ca"; // random

pub struct Terraria {
    out_buffer: Vec<u8>,
    stream: TcpStream,
}

const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

fn as_hex(buf: &[u8]) -> String {
    let buf = &buf[..buf.len().min(80)];
    let mut bytes = Vec::with_capacity(buf.len() * 2);
    buf.into_iter().for_each(|b| {
        bytes.push(HEX_DIGITS[(b >> 4) as usize]);
        bytes.push(HEX_DIGITS[(b & 15) as usize]);
    });
    unsafe { String::from_utf8_unchecked(bytes) }
}

impl Terraria {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        // connection
        let stream = TcpStream::connect(addr)?;
        let mut this = Self {
            out_buffer: vec![0; 1024],
            stream,
        };

        // handshake
        this.send_packet(&packets::Connect {
            version: PROTOCOL_VERSION.to_string(),
        })?;

        // TODO
        //this.send_packet(&packets::PlayerInfo::default())?;

        // TODO
        this.send_packet(&packets::ClientUuid {})?;

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
            bufftype: [0u16; 22],
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

        this.send_packet(&packets::RequestEssentialTiles { spawn_x: -1, spawn_y: -1 })?;

        this.send_packet(&packets::SpawnPlayer {
            player_id: 0,
            spawn_x: -1,
            spawn_y: -1,
            respawn_time_remaining: 0,
            player_spawn_context: 1,
        })?;

        // some more stuff to send at this point
        // > 82 : 0c005206000e00000000003f
        // > 56 : 0500380100
        // > 36 : 0800240000004200

        /*
        let (mut x, y) = (33534.0f32, 4582.0f32);
        loop {
            this.try_recv_packets()?;

            x -= 0.5;
            this.send_packet(&packets::PlayerMove {
                id: 0,
                flags: 0,
                speed_flags: 0,
                c: 0,
                d: 0,
                hotbar: 0,
                pos: Vec2 { x, y },
                vel: None,
            })?;
        }
        */
        Ok(this)
    }

    pub fn send_packet<P: PacketBody>(&mut self, packet: &P) -> io::Result<()> {
        let mut cursor = SliceCursor::new(self.out_buffer.as_mut_slice());
        packet.serialize(&mut cursor);
        let pos = cursor.finish();
        self.stream.write_all(&self.out_buffer[..pos])?;
        self.stream.flush()?;
        println!("> {} : {}", P::TAG, as_hex(&self.out_buffer[..pos]));

        // recv during send to see it real time
        self.try_recv_packets()?;

        Ok(())
    }

    pub fn try_recv_packets(&mut self) -> io::Result<()> {
        self.stream
            .set_read_timeout(Some(std::time::Duration::from_millis(10)))?;
        loop {
            match self.recv_packet() {
                Ok(_) => continue,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    break Ok(());
                }
                e => break e.map(drop),
            }
        }
    }

    pub fn recv_packet(&mut self) -> io::Result<Packet> {
        // TODO this is very inefficient
        let mut lenbuf = [0u8; 2];
        self.stream.read_exact(&mut lenbuf)?;
        let mut cursor = SliceCursor::new(&mut lenbuf);
        let len = cursor.read::<u16>() as usize;
        cursor.finish();

        let mut packet = vec![0u8; len - 2];
        self.stream.read_exact(&mut packet)?;

        println!("< {} : {}{}", packet[0], as_hex(&lenbuf), as_hex(&packet));
        Ok(Packet::from_slice(&mut packet))
    }
}
