use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 73. No information known yet.
#[derive(Debug)]
pub struct Packet73 {}

impl PacketBody for Packet73 {
    const TAG: u8 = 73;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
