use crate::packets::PacketBody;
use crate::SliceCursor;

/// Poof of smoke.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct PoofOfSmoke {
    /// Two Int16's packed into 4 bytes.
    pub packed_vector: u32,
}

impl PacketBody for PoofOfSmoke {
    const TAG: u8 = 106;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.packed_vector);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            packed_vector: cursor.read(),
        }
    }
}
