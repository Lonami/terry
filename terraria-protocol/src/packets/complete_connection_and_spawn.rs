use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Complete the connection process and spawn.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct CompleteConnectionAndSpawn {}

impl PacketBody for CompleteConnectionAndSpawn {
    const TAG: u8 = 49;

    fn write_body(&self, cursor: &mut SliceCursor) {}

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
