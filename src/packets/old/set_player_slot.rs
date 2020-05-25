use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Changes the user's slot to a different player identifier.
///
/// Direction: Server to Client.
#[derive(Debug)]
pub struct SetPlayerSlot {
    pub id: u8,
}

impl PacketBody for SetPlayerSlot {
    const TAG: u8 = 3;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self { id: cursor.read() }
    }
}
