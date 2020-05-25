use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// NPC Shop Item.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct NPCShopItem {
    pub slot: u8,
    pub item_type: i16,
    pub stack: i16,
    pub prefix: u8,
    pub value: i32,
    /// BitFlags: 1 = BuyOnce
    pub flags: u8,
}

impl PacketBody for NPCShopItem {
    const TAG: u8 = 104;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.slot);
        cursor.write(&self.item_type);
        cursor.write(&self.stack);
        cursor.write(&self.prefix);
        cursor.write(&self.value);
        cursor.write(&self.flags);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            slot: cursor.read(),
            item_type: cursor.read(),
            stack: cursor.read(),
            prefix: cursor.read(),
            value: cursor.read(),
            flags: cursor.read(),
        }
    }
}
