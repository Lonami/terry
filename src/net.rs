use crate::packets::{self, Packet, PacketBody, Vec2, RGB};
use crate::serialization::SliceCursor;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

const PROTOCOL_MAGIC: &str = "Terraria228";

// TODO don't use constants for these
const PLAYER_NAME: &str = "terry";
const PLAYER_UUID: &str = "01032c81-623f-4435-85e5-e0ec816b09ca"; // random

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
        this.send_packet(&packets::Magic {
            magic: PROTOCOL_MAGIC.to_string(),
        })?;

        this.send_packet(&packets::PlayerInfo {
            player: 0,
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

        this.send_packet(&packets::PlayerLife {
            id: 0,
            life: 100,
            max_life: 100,
        })?;

        this.send_packet(&packets::PlayerMana {
            id: 0,
            mana: 200,
            max_mana: 200,
        })?;

        this.send_packet(&packets::PlayerBuffs {
            id: 0,
            buffs: [0u16; 22],
        })?;

        for i in 0..260 {
            this.send_packet(&packets::PlayerInventory {
                id: 0,
                index: i,
                count: 0,
                a: 0,
                item_id: 0,
            })?;
        }

        this.send_packet(&packets::Packet6 {})?;

        this.send_packet(&packets::Packet8 { n: -1 })?;

        this.send_packet(&packets::ToSpawn {
            id: 0,
            x: -1,
            y: -1,
            timer: 0,
            how: 1,
        })?;

        // some more stuff to send at this point
        // > 82 : 0c005206000e00000000003f
        // > 56 : 0500380100
        // > 36 : 0800240000004200

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
