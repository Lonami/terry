use crate::packets::Packet;

/// Magic sent at the very beginning of the communication.
pub struct Magic {
    pub magic: String,
}

impl Packet for Magic {
    const TAG: u8 = 1;

    fn append_body(&self, buf: &mut Vec<u8>) {
        buf.extend(self.magic.as_bytes());
    }
}
