use crate::packets::PacketBody;
use crate::SliceCursor;

/// Emoji.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct Emoji {
    pub player_id: u8,
    pub emoticon: u8,
}

impl PacketBody for Emoji {
    const TAG: u8 = 120;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.emoticon);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            emoticon: cursor.read(),
        }
    }
}
