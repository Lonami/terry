use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Complete Angler Quest Today.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct CompleteAnglerQuestToday {
}

impl PacketBody for CompleteAnglerQuestToday {
    const TAG: u8 = 75;

    fn write_body(&self, cursor: &mut SliceCursor) {
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
        }
    }
}
