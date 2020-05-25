use crate::packets::PacketBody;
use crate::SliceCursor;

/// Placeholder.
///
/// Direction: Does not exist in the official client. Exists solely for the purpose of being used by custom clients and servers..
#[derive(Debug)]
pub struct Placeholder {}

impl PacketBody for Placeholder {
    const TAG: u8 = 67;

    fn write_body(&self, cursor: &mut SliceCursor) {}

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
