use crate::packets::PacketBody;
use crate::SliceCursor;

/// Set the user's slot.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SetUserSlot {
    pub player_id: u16,
}

impl PacketBody for SetUserSlot {
    const TAG: u8 = 3;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
        }
    }
}
