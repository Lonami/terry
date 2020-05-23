use crate::packets::Packet;

/// Packet 8, used during login. Seems to always be -1.
pub struct Packet8 {
    pub n: i32,
}

impl Packet for Packet8 {
    const TAG: u8 = 8;

    fn append_body(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.n.to_le_bytes());
    }
}
