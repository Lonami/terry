use crate::packets::PacketBody;
use crate::SliceCursor;

/// Toggle the gem lock.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct GemLockToggle {
    pub x: i16,
    pub y: i16,
    pub on: bool,
}

impl PacketBody for GemLockToggle {
    const TAG: u8 = 105;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.on);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            on: cursor.read(),
        }
    }
}
