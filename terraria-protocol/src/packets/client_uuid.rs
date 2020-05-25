use crate::packets::PacketBody;
use crate::SliceCursor;

/// Client UUID.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ClientUuid {}

impl PacketBody for ClientUuid {
    const TAG: u8 = 68;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
