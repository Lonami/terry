use crate::serde::{PacketBody, Result, SliceCursor};
use std::fmt;

/// Travelling Merchant Inventory.
///
/// Direction: Server -> Client.
#[derive(Clone)]
pub struct TravellingMerchantInventory {
    /// Each short related to an item type NetID.
    pub items: [i16; 40],
}

impl PacketBody for TravellingMerchantInventory {
    const TAG: u8 = 72;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        for i in self.items.iter() {
            cursor.write(i)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let mut items = [0; 40];
        for i in items.iter_mut() {
            *i = cursor.read()?;
        }
        Ok(Self { items })
    }
}

// Can't derive because the length of arrays must be at most 32 for that.
impl fmt::Debug for TravellingMerchantInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.items.iter()).finish()
    }
}

impl PartialEq for TravellingMerchantInventory {
    fn eq(&self, other: &Self) -> bool {
        self.items
            .iter()
            .zip(other.items.iter())
            .all(|(a, b)| a == b)
    }
}

impl Eq for TravellingMerchantInventory {}

impl Default for TravellingMerchantInventory {
    fn default() -> Self {
        Self { items: [0; 40] }
    }
}
