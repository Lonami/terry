use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update Item Owner.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdateItemOwner {
    pub item_id: i16,
    pub player_id: u8,
}

impl PacketBody for UpdateItemOwner {
    const TAG: u8 = 22;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.item_id);
        cursor.write(&self.player_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            item_id: cursor.read(),
            player_id: cursor.read(),
        }
    }
}
