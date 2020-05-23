use crate::packets::Packet;

/// Current and maximum mana of the player.
pub struct PlayerMana {
    pub mana: u16,
    pub max_mana: u16,
}

impl Packet for PlayerMana {
    const TAG: u8 = 42;

    fn append_body(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.mana.to_le_bytes());
        buf.extend(&self.max_mana.to_le_bytes());
    }
}
