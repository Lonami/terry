use crate::packets::PacketBody;
use crate::SliceCursor;

/// Open a chest.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct OpenChest {
    pub tile_x: i16,
    pub tile_y: i16,
}

impl PacketBody for OpenChest {
    const TAG: u8 = 31;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.tile_x);
        cursor.write(&self.tile_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            tile_x: cursor.read(),
            tile_y: cursor.read(),
        }
    }
}
