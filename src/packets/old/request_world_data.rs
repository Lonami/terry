use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Asks the server to send the data about the world.
///
/// Direction: Client to Server.
#[derive(Debug)]
pub struct RequestWorldData {}

impl PacketBody for RequestWorldData {
    const TAG: u8 = 6;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
