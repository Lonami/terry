use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update Chest Item.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdateChestItem {
    pub chest_id: i16,
    pub item_slot: u8,
    pub stack: i16,
    pub prefix: u8,
    pub item_net_id: i16,
}

impl PacketBody for UpdateChestItem {
    const TAG: u8 = 32;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.chest_id);
        cursor.write(&self.item_slot);
        cursor.write(&self.stack);
        cursor.write(&self.prefix);
        cursor.write(&self.item_net_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            chest_id: cursor.read(),
            item_slot: cursor.read(),
            stack: cursor.read(),
            prefix: cursor.read(),
            item_net_id: cursor.read(),
        }
    }
}
