use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update Minion Target.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdateMinionTarget {
    pub player_id: u8,
    pub target_x: i32, /* single */
    pub target_y: i32, /* single */
}

impl PacketBody for UpdateMinionTarget {
    const TAG: u8 = 99;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.target_x);
        cursor.write(&self.target_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            target_x: cursor.read(),
            target_y: cursor.read(),
        }
    }
}
