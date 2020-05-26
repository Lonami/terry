use crate::packets::PacketBody;
use crate::SliceCursor;

/// Client finished inventory changes on this tick.
///
/// It's sent by the client code twice when a player moves an item around in
/// their inventory, although the packet has no data.
///
/// The total payload size is 2 packets per inventory item drag, with 3 bytes
/// each (2 for length, 1 for packet ID). This is a functionally useless
/// packet.
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
