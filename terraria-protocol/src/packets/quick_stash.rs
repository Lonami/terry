use crate::packets::PacketBody;
use crate::SliceCursor;

/// Force an item into the nearest chest (quick stashing).
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct QuickStash {
    pub inventory_slot: u8,
}

impl PacketBody for QuickStash {
    const TAG: u8 = 85;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.inventory_slot);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            inventory_slot: cursor.read(),
        }
    }
}
