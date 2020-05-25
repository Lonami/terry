use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Create a temporary animation.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct CreateTemporaryAnimation {
    pub animation_type: i16,
    pub tile_type: u16,
    pub x: i16,
    pub y: i16,
}

impl PacketBody for CreateTemporaryAnimation {
    const TAG: u8 = 77;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.animation_type);
        cursor.write(&self.tile_type);
        cursor.write(&self.x);
        cursor.write(&self.y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            animation_type: cursor.read(),
            tile_type: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
        }
    }
}
