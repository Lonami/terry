use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Nebula Level Up.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct NebulaLevelUp {
    pub player_id: u8,
    pub level_up_type: u16,
    /// In world coordinate pixels.
    pub origin_x: i32, /* single */
    /// In world coordinate pixels.
    pub origin_y: i32, /* single */
}

impl PacketBody for NebulaLevelUp {
    const TAG: u8 = 102;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.level_up_type);
        cursor.write(&self.origin_x);
        cursor.write(&self.origin_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            level_up_type: cursor.read(),
            origin_x: cursor.read(),
            origin_y: cursor.read(),
        }
    }
}
