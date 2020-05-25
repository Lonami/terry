use crate::packets::PacketBody;
use crate::SliceCursor;

/// Request the essential tiles.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct RequestEssentialTiles {
    /// Player Spawn X
    pub x: i32,
    /// Player Spawn Y
    pub y: i32,
}

impl PacketBody for RequestEssentialTiles {
    const TAG: u8 = 8;

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
