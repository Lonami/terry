use crate::packets::PacketBody;
use crate::SliceCursor;

/// A certain player has died.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct DeadPlayer {
    pub player_id: u8,
}

impl PacketBody for DeadPlayer {
    const TAG: u8 = 135;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
        }
    }
}
