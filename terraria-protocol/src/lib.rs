//! This library contains all the packet definitions needed to work with
//! Terraria's multiplayer network protocol, and it can be used to build
//! a client, a robot, or even a custom Terraria server.
//!
//! Note that the library itself only contains the packets and mechanisms
//! to serialize and deserialize them, along with basic protocol management.
//! If you want it to do anything interesting, you should build that yourself.
pub mod net;
pub mod packets;
pub mod structures;

pub(crate) use structures::{Deserializable, Serializable, SliceCursor};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_remote_deserialization() {
        let mut data = include_bytes!("../test-data/remote.bin").to_vec();

        let mut index = 0;
        while index < data.len() {
            // TODO don't reimplement this here
            let packet_len = u16::from_le_bytes([data[index], data[index + 1]]) as usize;
            packets::Packet::from_slice(&mut data[index + 2..index + packet_len]);
            index += packet_len;
        }

        todo!("make sure we found a certain number of present tiles")
    }
}
