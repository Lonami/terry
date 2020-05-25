use crate::packets::PacketBody;
use crate::structures::Vec2;
use crate::SliceCursor;

/// Update Minion Target.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdateMinionTarget {
    pub player_id: u8,
    pub target: Vec2,
}

impl PacketBody for UpdateMinionTarget {
    const TAG: u8 = 99;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.target);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            target: cursor.read(),
        }
    }
}
