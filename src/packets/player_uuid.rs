use crate::packets::Packet;

/// Player UUID-4.
pub struct PlayerUuid {
    pub uuid4: String,
}

impl Packet for PlayerUuid {
    const TAG: u8 = 68;

    fn append_body(&self, buf: &mut Vec<u8>) {
        buf.extend(self.uuid4.as_bytes());
    }
}
