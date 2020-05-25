use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Hit a switch.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct HitSwitch {
    pub x: i16,
    pub y: i16,
}

impl PacketBody for HitSwitch {
    const TAG: u8 = 59;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
        }
    }
}
