use crate::packets::PacketBody;
use crate::SliceCursor;

/// Complete the Angler quest today.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct CompleteAnglerQuest {}

impl PacketBody for CompleteAnglerQuest {
    const TAG: u8 = 75;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
