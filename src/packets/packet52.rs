use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 52. No information known yet.
#[derive(Debug)]
pub struct Packet52 {}

impl PacketBody for Packet52 {
    const TAG: u8 = 52;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
