use crate::packets::PacketBody;
use crate::SliceCursor;

/// Sync an emote bubble.
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SyncEmoteBubble {
    pub emote_id: i32,
    pub anchor_type: u8,
    pub player_id: Option<u16>,
    pub emote_lifetime: Option<u16>,
    pub emote: Option<u8>,
    pub emote_metadata: Option<i16>,
}

impl PacketBody for SyncEmoteBubble {
    const TAG: u8 = 91;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.emote_id);
        cursor.write(&self.anchor_type);

        if self.anchor_type != 255 {
            cursor.write(&self.player_id.unwrap());
            cursor.write(&self.emote_lifetime.unwrap());
            cursor.write(&self.emote.unwrap());
            if self.emote.unwrap() & 0x80 != 0 {
                cursor.write(&self.emote_metadata.unwrap());
            }
        }
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let emote_id = cursor.read();
        let anchor_type = cursor.read();

        let (player_id, emote_lifetime, emote) = if anchor_type != 255 {
            (
                Some(cursor.read()),
                Some(cursor.read()),
                Some(cursor.read()),
            )
        } else {
            (None, None, None)
        };

        let mut emote_metadata = None;
        if let Some(emote) = emote {
            if emote & 0x80 != 0 {
                emote_metadata = Some(cursor.read());
            }
        }

        Self {
            emote_id,
            anchor_type,
            player_id,
            emote_lifetime,
            emote,
            emote_metadata,
        }
    }
}
