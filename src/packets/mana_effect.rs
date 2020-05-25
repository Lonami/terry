use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Apply a mana healing effect.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct ManaEffect {
    pub player_id: u8,
    pub mana_amount: i16,
}

impl PacketBody for ManaEffect {
    const TAG: u8 = 43;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.mana_amount);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            mana_amount: cursor.read(),
        }
    }
}
