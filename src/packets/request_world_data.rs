use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Request world data.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct RequestWorldData {
}

impl PacketBody for RequestWorldData {
    const TAG: u8 = 6;

    fn write_body(&self, cursor: &mut SliceCursor) {
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
        }
    }
}
