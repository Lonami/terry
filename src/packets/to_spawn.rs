use crate::packets::Packet;

/// Goes to spawn, either by login, death or recall.
pub struct ToSpawn {
    pub x: i16,
    pub y: i16,
    pub timer: u32,
    pub how: u8,
}

impl Packet for ToSpawn {
    const TAG: u8 = 12;

    fn append_body(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.x.to_le_bytes());
        buf.extend(&self.y.to_le_bytes());
        buf.extend(&self.timer.to_le_bytes());
        buf.push(self.how);
    }
}
