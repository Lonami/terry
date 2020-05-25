use crate::packets::PacketBody;
use crate::SliceCursor;

/// Request a sign.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct RequestSign {
    pub x: i16,
    pub y: i16,
}

impl PacketBody for RequestSign {
    const TAG: u8 = 46;

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
