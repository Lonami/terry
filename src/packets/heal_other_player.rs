use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Heal Other Player.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct HealOtherPlayer {
    pub player_id: u8,
    pub heal_amount: i16,
}

impl PacketBody for HealOtherPlayer {
    const TAG: u8 = 66;

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
