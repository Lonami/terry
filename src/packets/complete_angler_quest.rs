use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Complete the Angler quest today.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct CompleteAnglerQuest {
}

impl PacketBody for CompleteAnglerQuest {
    const TAG: u8 = 75;

    fn write_body(&self, cursor: &mut SliceCursor) {
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
        }
    }
}
