use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Items in the player's inventory.
#[derive(Debug)]
pub struct PlayerInventory {
    pub id: u8,
    pub index: u16,
    pub count: u16,
    pub a: u8, // ???
    pub item_id: u16,
}

impl PacketBody for PlayerInventory {
    const TAG: u8 = 5;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        cursor.write(&self.index);
        cursor.write(&self.count);
        cursor.write(&self.a);
        cursor.write(&self.item_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            id: cursor.read(),
            index: cursor.read(),
            count: cursor.read(),
            a: cursor.read(),
            item_id: cursor.read(),
        }
    }
}
