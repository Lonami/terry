use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 6 is used during login.
#[derive(Debug)]
pub struct Packet6 {}

impl PacketBody for Packet6 {
    const TAG: u8 = 6;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
