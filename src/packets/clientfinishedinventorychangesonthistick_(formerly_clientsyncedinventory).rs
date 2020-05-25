use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// ClientFinishedInventoryChangesOnThisTick (formerly ClientSyncedInventory).
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ClientFinishedInventoryChangesOnThisTick(formerlyClientSyncedInventory) {
}

impl PacketBody for ClientFinishedInventoryChangesOnThisTick(formerlyClientSyncedInventory) {
    const TAG: u8 = 138;

    fn write_body(&self, cursor: &mut SliceCursor) {
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
        }
    }
}
