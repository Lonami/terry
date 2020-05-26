use crate::packets::PacketBody;
use crate::SliceCursor;

/// Poof of smoke.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct PoofOfSmoke {
    pub x: i16,
    pub y: i16,
}

impl PacketBody for PoofOfSmoke {
    const TAG: u8 = 106;

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
