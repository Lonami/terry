use crate::packets::PacketBody;
use crate::SliceCursor;
use crate::structures::Vec2;

/// Nebula Level Up.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct NebulaLevelUp {
    pub player_id: u8,
    pub level_up_type: u16,
    /// In world coordinate pixels.
    pub origin: Vec2,
}

impl PacketBody for NebulaLevelUp {
    const TAG: u8 = 102;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.level_up_type);
        cursor.write(&self.origin);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            level_up_type: cursor.read(),
            origin: cursor.read(),
        }
    }
}
