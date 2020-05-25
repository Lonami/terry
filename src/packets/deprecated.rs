use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Deprecated.
///
/// Direction: Not used.
#[derive(Debug)]
pub struct Deprecated {
}

impl PacketBody for Deprecated {
    const TAG: u8 = 94;

    fn write_body(&self, cursor: &mut SliceCursor) {
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
        }
    }
}
