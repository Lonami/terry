use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 95. No information known yet.
#[derive(Debug)]
pub struct Packet95 {}

impl PacketBody for Packet95 {
    const TAG: u8 = 95;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
