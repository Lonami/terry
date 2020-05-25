use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Sync Emote Bubble.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SyncEmoteBubble {
    pub emote_id: i32,
    pub anchor_type: u8,
    /// Only sent if AnchorType != 255
    pub player_id: u16,
    /// Only sent if AnchorType != 255
    pub emote_lifetime: u16,
    /// Only sent if AnchorType != 255
    pub emote: u8,
    /// Only sent if AnchorType != 255 and Emote &lt; 0
    pub emote_metadata: i16,
}

impl PacketBody for SyncEmoteBubble {
    const TAG: u8 = 91;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.emote_id);
        cursor.write(&self.anchor_type);
        cursor.write(&self.player_id);
        cursor.write(&self.emote_lifetime);
        cursor.write(&self.emote);
        cursor.write(&self.emote_metadata);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            emote_id: cursor.read(),
            anchor_type: cursor.read(),
            player_id: cursor.read(),
            emote_lifetime: cursor.read(),
            emote: cursor.read(),
            emote_metadata: cursor.read(),
        }
    }
}
