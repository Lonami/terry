use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Remove Item Owner.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct RemoveItemOwner {
    pub item_index: i16,
}

impl PacketBody for RemoveItemOwner {
    const TAG: u8 = 39;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.item_index);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            item_index: cursor.read(),
        }
    }
}
