use crate::packets::PacketBody;
use crate::SliceCursor;
use std::fmt;

/// Travelling Merchant Inventory.
///
/// Direction: Server -> Client.
pub struct TravellingMerchantInventory {
    /// Each short related to an item type NetID.
    pub items: [i16; 40],
}

impl PacketBody for TravellingMerchantInventory {
    const TAG: u8 = 72;

    fn write_body(&self, cursor: &mut SliceCursor) {
        self.items.iter().for_each(|i| cursor.write(i));
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let mut items = [0; 40];
        items.iter_mut().for_each(|i| *i = cursor.read());
        Self { items }
    }
}

// Can't derive because the length of arrays must be at most 32 for that.
impl fmt::Debug for TravellingMerchantInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.items.iter()).finish()
    }
}
