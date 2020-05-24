use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 54. No information known yet.
#[derive(Debug)]
pub struct Packet54 {}

impl PacketBody for Packet54 {
    const TAG: u8 = 54;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
