use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Paint Wall.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PaintWall {
    pub x: i16,
    pub y: i16,
    pub color: u8,
}

impl PacketBody for PaintWall {
    const TAG: u8 = 64;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.color);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            color: cursor.read(),
        }
    }
}
