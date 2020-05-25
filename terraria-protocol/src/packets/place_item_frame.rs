use crate::packets::PacketBody;
use crate::SliceCursor;

/// Place an item frame.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct PlaceItemFrame {
    pub x: i16,
    pub y: i16,
    pub item_id: i16,
    pub prefix: u8,
    pub stack: i16,
}

impl PacketBody for PlaceItemFrame {
    const TAG: u8 = 89;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.item_id);
        cursor.write(&self.prefix);
        cursor.write(&self.stack);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            item_id: cursor.read(),
            prefix: cursor.read(),
            stack: cursor.read(),
        }
    }
}
