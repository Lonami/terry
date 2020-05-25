use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Force Item Into Nearest Chest.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ForceItemIntoNearestChest {
    pub inventory_slot: u8,
}

impl PacketBody for ForceItemIntoNearestChest {
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
