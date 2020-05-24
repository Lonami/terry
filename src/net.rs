use crate::packets::{self, Packet, PacketBody};
use crate::serialization::SliceCursor;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

const PROTOCOL_MAGIC: &str = "Terraria228";

// TODO don't use constants for these
const PLAYER_NAME: &str = "terry";
const PLAYER_UUID: &str = "01032c81-623f-4435-85e5-e0ec816b09ca"; // random

pub struct Terraria {
    player: u8,
    in_buffer: Vec<u8>,
    out_buffer: Vec<u8>,
    stream: TcpStream,
}

impl Terraria {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        // connection
        let stream = TcpStream::connect(addr)?;
        let mut this = Self {
            player: 0,
            in_buffer: vec![0; 1024],
            out_buffer: vec![0; 1024],
            stream,
        };

        // handshake
        this.send_packet(&packets::Magic { magic: PROTOCOL_MAGIC.to_string() })?;

        // TODO continue with the rest of the handshake

        Ok(this)
    }

    pub fn send_packet<P: PacketBody>(&mut self, packet: &P) -> io::Result<()> {
        let mut cursor = SliceCursor::new(self.out_buffer.as_mut_slice());
        packet.serialize(self.player, &mut cursor);
        let pos = cursor.finish();
        self.stream.write_all(&self.out_buffer[..pos])?;
        Ok(())
    }

    pub fn recv_packet() -> io::Result<Packet> {
        todo!()
    }
}
