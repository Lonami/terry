use crate::packets::PacketBody;
use crate::SliceCursor;

/// Get a chest's name.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct GetChestName {
    pub id: i16,
    pub x: i16,
    pub y: i16,
    pub name: String,
}

impl PacketBody for GetChestName {
    const TAG: u8 = 69;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.name);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            id: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            name: cursor.read(),
        }
    }
}
