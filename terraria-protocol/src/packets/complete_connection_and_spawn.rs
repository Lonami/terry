use crate::packets::PacketBody;
use crate::SliceCursor;

/// Complete the connection process and spawn.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct CompleteConnectionAndSpawn {}

impl PacketBody for CompleteConnectionAndSpawn {
    const TAG: u8 = 49;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
