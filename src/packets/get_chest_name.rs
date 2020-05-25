use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Get Chest Name.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct GetChestName {
    pub chest_id: i16,
    pub chest_x: i16,
    pub chest_y: i16,
    pub name: String,
}

impl PacketBody for GetChestName {
    const TAG: u8 = 69;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.chest_id);
        cursor.write(&self.chest_x);
        cursor.write(&self.chest_y);
        cursor.write(&self.name);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            chest_id: cursor.read(),
            chest_x: cursor.read(),
            chest_y: cursor.read(),
            name: cursor.read(),
        }
    }
}
