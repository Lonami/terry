use crate::packets::PacketBody;
use crate::SliceCursor;

/// Finished connecting to the server.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct ConnectionComplete {}

impl PacketBody for ConnectionComplete {
    const TAG: u8 = 129;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
