use crate::packets::PacketBody;
use crate::SliceCursor;

/// The item from a NPC shop.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct NpcShopItem {
    pub slot: u8,
    pub item_type: i16,
    pub stack: i16,
    pub prefix: u8,
    pub value: i32,
    pub buy_once: bool,
}

impl PacketBody for NpcShopItem {
    const TAG: u8 = 104;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.slot);
        cursor.write(&self.item_type);
        cursor.write(&self.stack);
        cursor.write(&self.prefix);
        cursor.write(&self.value);
        cursor.write(&self.buy_once);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            slot: cursor.read(),
            item_type: cursor.read(),
            stack: cursor.read(),
            prefix: cursor.read(),
            value: cursor.read(),
            buy_once: cursor.read(),
        }
    }
}
