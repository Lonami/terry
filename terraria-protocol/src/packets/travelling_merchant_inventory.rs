use crate::packets::PacketBody;
use crate::SliceCursor;

/// Travelling Merchant Inventory.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct TravellingMerchantInventory {
    /// Each short related to an item type NetID.
    pub items: i16,
}

impl PacketBody for TravellingMerchantInventory {
    const TAG: u8 = 72;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.items);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            items: cursor.read(),
        }
    }
}
