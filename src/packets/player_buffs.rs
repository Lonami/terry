use crate::packets::Packet;

const BUFF_COUNT: usize = 22;

/// Player buffs and debuffs.
pub struct PlayerBuffs {
    pub buffs: Vec<u16>,
}

impl Packet for PlayerBuffs {
    const TAG: u8 = 50;

    fn append_body(&self, buf: &mut Vec<u8>) {
        assert!(self.buffs.len() <= BUFF_COUNT, "too many buffs");
        for i in 0..BUFF_COUNT {
            buf.extend(&self.buffs.get(i).unwrap_or(&0).to_le_bytes());
        }
    }
}
