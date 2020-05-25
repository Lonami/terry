use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Current and maximum mana of the player.
#[derive(Debug)]
pub struct PlayerMana {
    pub id: u8,
    pub mana: u16,
    pub max_mana: u16,
}

impl PacketBody for PlayerMana {
    const TAG: u8 = 42;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        cursor.write(&self.mana);
        cursor.write(&self.max_mana);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            id: cursor.read(),
            mana: cursor.read(),
            max_mana: cursor.read(),
        }
    }
}
