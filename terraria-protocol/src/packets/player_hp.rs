use crate::packets::PacketBody;
use crate::SliceCursor;

/// Player health points and maximum HP.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerHP {
    pub player_id: u8,
    pub hp: i16,
    pub max_hp: i16,
}

impl PacketBody for PlayerHP {
    const TAG: u8 = 16;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.hp);
        cursor.write(&self.max_hp);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            hp: cursor.read(),
            max_hp: cursor.read(),
        }
    }
}
