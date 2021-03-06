use crate::packets::PacketBody;
use crate::SliceCursor;

/// Number of Angler quests completed.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct AnglerQuests {
    pub player_id: u8,
    pub angler_quests_completed: i32,
    pub golfer_score: i32,
}

impl PacketBody for AnglerQuests {
    const TAG: u8 = 76;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.angler_quests_completed);
        cursor.write(&self.golfer_score);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            angler_quests_completed: cursor.read(),
            golfer_score: cursor.read(),
        }
    }
}
