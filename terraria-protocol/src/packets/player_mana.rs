use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Player mana and maximum mana.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerMana {
    pub player_id: u8,
    pub mana: i16,
    pub max_mana: i16,
}

impl PacketBody for PlayerMana {
    const TAG: u8 = 42;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.mana);
        cursor.write(&self.max_mana);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            mana: cursor.read(),
            max_mana: cursor.read(),
        }
    }
}
