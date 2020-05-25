use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Send Tile Square.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SendTileSquare {
    pub size: u16,
    /// Only if (Size &amp; 0x8000) != 0
    pub tilechangetype: u8,
    pub tile_x: i16,
    pub tile_y: i16,
}

impl PacketBody for SendTileSquare {
    const TAG: u8 = 20;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.size);
        cursor.write(&self.tilechangetype);
        cursor.write(&self.tile_x);
        cursor.write(&self.tile_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            size: cursor.read(),
            tilechangetype: cursor.read(),
            tile_x: cursor.read(),
            tile_y: cursor.read(),
        }
    }
}
