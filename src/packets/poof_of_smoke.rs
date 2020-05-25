use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Poof of Smoke.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct PoofofSmoke {
    /// Two Int16's packed into 4 bytes.
    pub packedvector: u32,
}

impl PacketBody for PoofofSmoke {
    const TAG: u8 = 106;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.packedvector);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            packedvector: cursor.read(),
        }
    }
}
