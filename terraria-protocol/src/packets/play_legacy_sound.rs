use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Play a legacy sound.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct PlayLegacySound {
    pub x: i32, /* single */
    pub y: i32, /* single */
    pub sound_id: u16,
    /// BitFlags: 1 = Style, 2 = Volume Scale, 3 = Pitch Offset
    pub sound_flags: u8,
}

impl PacketBody for PlayLegacySound {
    const TAG: u8 = 132;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.sound_id);
        cursor.write(&self.sound_flags);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            sound_id: cursor.read(),
            sound_flags: cursor.read(),
        }
    }
}
