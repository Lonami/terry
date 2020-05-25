use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// FinishedConnectingToServer.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct FinishedConnectingToServer {
}

impl PacketBody for FinishedConnectingToServer {
    const TAG: u8 = 129;

    fn write_body(&self, cursor: &mut SliceCursor) {
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
        }
    }
}
