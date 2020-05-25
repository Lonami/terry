use crate::packets::PacketBody;
use crate::SliceCursor;

/// Request password.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct RequestPassword {}

impl PacketBody for RequestPassword {
    const TAG: u8 = 37;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
