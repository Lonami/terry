use crate::packets::{self, Packet, PacketBody, RGB};
use crate::serialization::SliceCursor;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

const PROTOCOL_MAGIC: &str = "Terraria228";

// TODO don't use constants for these
const PLAYER_NAME: &str = "terry";
const PLAYER_UUID: &str = "01032c81-623f-4435-85e5-e0ec816b09ca"; // random

pub struct Terraria {
    player: u8,
    out_buffer: Vec<u8>,
    stream: TcpStream,
}

const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

fn as_hex(buf: &[u8]) -> String {
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
            player: 0,
            out_buffer: vec![0; 1024],
            stream,
        };

        // handshake
        this.send_packet(&packets::Magic {
            magic: PROTOCOL_MAGIC.to_string(),
        })?;

        this.send_packet(&packets::PlayerInfo {
            skin_variant: 0,
            hair_variant: 0,
            name: PLAYER_NAME.to_string(),
            hair_dye: 0,
            visible_accesories_flags: 0,
            hide_misc: false,
            hair_color: RGB::new(),
            skin_color: RGB::new(),
            eye_color: RGB::new(),
            shirt_color: RGB::new(),
            undershirt_color: RGB::new(),
            pants_color: RGB::new(),
            shoes_color: RGB::new(),
            difficulty_flags: 4,
        })?;

        this.send_packet(&packets::PlayerUuid {
            uuid4: PLAYER_UUID.to_string(),
        })?;

        this.send_packet(&packets::PlayerMana {
            mana: 200,
            max_mana: 200,
        })?;
        this.send_packet(&packets::PlayerBuffs { buffs: [0u16; 22] })?;

        // TODO 6

        this.send_packet(&packets::Packet8 { n: -1 })?;

        this.send_packet(&packets::ToSpawn {
            x: -1,
            y: -1,
            timer: 0,
            how: 1,
        })?;

        // 127.0.0.1:41124 was booted: Invalid operation at this state.

        loop {
            match this.recv_packet()? {
                Packet::Packet82(_) => eprintln!("Packet82"),
                packet => {
                    dbg!(packet);
                }
            };
        }

        Ok(this)
    }

    pub fn send_packet<P: PacketBody>(&mut self, packet: &P) -> io::Result<()> {
        let mut cursor = SliceCursor::new(self.out_buffer.as_mut_slice());
        packet.serialize(None, &mut cursor);
        let pos = cursor.finish();
        self.stream.write_all(&self.out_buffer[..pos])?;
        self.stream.flush()?;
        println!("> {} : {}", P::TAG, as_hex(&self.out_buffer[..pos]));
        Ok(())
    }

    pub fn recv_packet(&mut self) -> io::Result<Packet> {
        // TODO this is very inefficient
        let mut len = [0u8; 2];
        self.stream.read_exact(&mut len)?;
        let mut cursor = SliceCursor::new(&mut len);
        let len = cursor.read::<u16>() as usize;
        cursor.finish();

        let mut packet = vec![0u8; len - 2];
        self.stream.read_exact(&mut packet)?;
        Ok(Packet::from_slice(&mut packet).1)
    }
}
