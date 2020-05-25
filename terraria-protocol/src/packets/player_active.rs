use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Player activity.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct PlayerActive {
    pub player_id: u8,
    pub active: bool,
}

impl PacketBody for PlayerActive {
    const TAG: u8 = 14;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.active);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            active: cursor.read(),
        }
    }
}
