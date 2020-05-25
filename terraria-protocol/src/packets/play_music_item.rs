use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Play a music item.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayMusicItem {
    pub player_id: u8,
    pub note: i32, /* single */
}

impl PacketBody for PlayMusicItem {
    const TAG: u8 = 58;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.note);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            note: cursor.read(),
        }
    }
}
