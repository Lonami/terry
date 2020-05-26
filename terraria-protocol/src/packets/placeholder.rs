use crate::packets::PacketBody;
use crate::SliceCursor;

/// Placeholder. Does not exist in the official client. Exists solely for
/// the purpose of being used by custom clients and servers.
///
/// Direction: Variable.
#[derive(Debug)]
pub struct Placeholder {}

impl PacketBody for Placeholder {
    const TAG: u8 = 67;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
