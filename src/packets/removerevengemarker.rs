use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// RemoveRevengeMarker.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct RemoveRevengeMarker {
    pub unique_id: i32,
}

impl PacketBody for RemoveRevengeMarker {
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
