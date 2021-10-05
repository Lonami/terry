//! This library contains all the packet definitions needed to work with
//! Terraria's multiplayer network protocol, and it can be used to build
//! a client, a robot, or even a custom Terraria server.
//!
//! Note that the library itself only contains the packets and mechanisms
//! to serialize and deserialize them, along with basic protocol management.
//! If you want it to do anything interesting, you should build that yourself.
pub mod net;
mod packet;
pub mod packets;
mod parser;
pub mod serde;
pub mod structures;
pub(crate) mod utils;

pub use packet::Packet;
pub use parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_remote_deserialization() {
        let mut data = include_bytes!("../../server-traffic.bin").to_vec();

        let mut index = 0;
        while index < data.len() {
            // TODO don't reimplement this here
            let packet_len = u16::from_le_bytes([data[index], data[index + 1]]) as usize;
            eprintln!(
                "checking tag {}: {}",
                data[index + 2],
                utils::HexString(&data[index + 3..index + packet_len])
            );
            let mut cursor = serde::SliceCursor::new(&mut data[index + 2..index + packet_len]);

            cursor.read::<Packet>().unwrap();
            index += packet_len;
        }
    }
}
