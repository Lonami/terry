use crate::packets::PacketBody;
use crate::SliceCursor;

/// Request world data.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct RequestWorldData {}

impl PacketBody for RequestWorldData {
    const TAG: u8 = 6;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
