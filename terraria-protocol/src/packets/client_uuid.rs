use crate::packets::PacketBody;
use crate::SliceCursor;

/// Client UUID.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ClientUuid {}

impl PacketBody for ClientUuid {
    const TAG: u8 = 68;

    fn write_body(&self, cursor: &mut SliceCursor) {}

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
