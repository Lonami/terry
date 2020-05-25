use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Place Item Frame.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct PlaceItemFrame {
    pub x: i16,
    pub y: i16,
    pub itemid: i16,
    pub prefix: u8,
    pub stack: i16,
}

impl PacketBody for PlaceItemFrame {
    const TAG: u8 = 89;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.itemid);
        cursor.write(&self.prefix);
        cursor.write(&self.stack);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            itemid: cursor.read(),
            prefix: cursor.read(),
            stack: cursor.read(),
        }
    }
}
