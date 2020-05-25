use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Section of a tile frame.
///
/// Direction: Server to Client.
#[derive(Debug)]
pub struct TileFrameSection {
    pub start_x: i16,
    pub start_y: i16,
    pub end_x: i16,
    pub end_y: i16,
}

impl PacketBody for TileFrameSection {
    const TAG: u8 = 11;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.start_x);
        cursor.write(&self.start_y);
        cursor.write(&self.end_x);
        cursor.write(&self.end_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            start_x: cursor.read(),
            start_y: cursor.read(),
            end_x: cursor.read(),
            end_y: cursor.read(),
        }
    }
}
