use crate::packets::PacketBody;
use crate::SliceCursor;

/// Finished connecting to the server.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct ConnectionComplete {}

impl PacketBody for ConnectionComplete {
    const TAG: u8 = 129;

    fn write_body(&self, cursor: &mut SliceCursor) {}

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
