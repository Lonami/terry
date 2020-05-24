use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 120. No information known yet.
#[derive(Debug)]
pub struct Packet120 {}

impl PacketBody for Packet120 {
    const TAG: u8 = 120;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
