use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 49, used during login.
#[derive(Debug)]
pub struct Packet49 {}

impl PacketBody for Packet49 {
    const TAG: u8 = 49;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
