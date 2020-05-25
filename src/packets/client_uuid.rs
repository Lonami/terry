use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Client UUID.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ClientUUID {
}

impl PacketBody for ClientUUID {
    const TAG: u8 = 68;

    fn write_body(&self, cursor: &mut SliceCursor) {
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
        }
    }
}
