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
