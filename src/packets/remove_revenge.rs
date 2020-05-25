use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Remove the revenge marker.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct RemoveRevenge {
    pub unique_id: i32,
}

impl PacketBody for RemoveRevenge {
    const TAG: u8 = 127;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.unique_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            unique_id: cursor.read(),
        }
    }
}
