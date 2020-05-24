use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 92. No information known yet.
#[derive(Debug)]
pub struct Packet92 {}

impl PacketBody for Packet92 {
    const TAG: u8 = 92;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
