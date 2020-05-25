use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Paint a tile.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PaintTile {
    pub x: i16,
    pub y: i16,
    pub color: u8,
}

impl PacketBody for PaintTile {
    const TAG: u8 = 63;

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
