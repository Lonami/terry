use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Apply a healing effect.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct HealEffect {
    pub player_id: u8,
    pub heal_amount: i16,
}

impl PacketBody for HealEffect {
    const TAG: u8 = 35;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.heal_amount);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            heal_amount: cursor.read(),
        }
    }
}
