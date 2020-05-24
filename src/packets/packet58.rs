use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 58. No information known yet.
#[derive(Debug)]
pub struct Packet58 {}

impl PacketBody for Packet58 {
    const TAG: u8 = 58;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
