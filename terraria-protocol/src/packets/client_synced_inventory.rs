use crate::packets::PacketBody;
use crate::SliceCursor;

/// Client finished inventory changes on this tick.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ClientSyncedInventory {}

impl PacketBody for ClientSyncedInventory {
    const TAG: u8 = 138;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
