use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 61. No information known yet.
#[derive(Debug)]
pub struct Packet61 {}

impl PacketBody for Packet61 {
    const TAG: u8 = 61;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
