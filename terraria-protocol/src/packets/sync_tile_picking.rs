use crate::packets::PacketBody;
use crate::SliceCursor;

/// Sync tile picking.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct SyncTilePicking {
    pub player_id: u8,
    pub x: i16,
    pub y: i16,
    pub pick_damage: u8,
}

impl PacketBody for SyncTilePicking {
    const TAG: u8 = 125;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.pick_damage);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            pick_damage: cursor.read(),
        }
    }
}
