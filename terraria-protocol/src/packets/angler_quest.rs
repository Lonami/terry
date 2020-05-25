use crate::packets::PacketBody;
use crate::SliceCursor;

/// Information about Angler quests.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct AnglerQuest {
    pub quest: u8,
    pub completed: bool,
}

impl PacketBody for AnglerQuest {
    const TAG: u8 = 74;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.quest);
        cursor.write(&self.completed);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            quest: cursor.read(),
            completed: cursor.read(),
        }
    }
}
