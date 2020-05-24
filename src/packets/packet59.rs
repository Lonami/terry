use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 59. No information known yet.
#[derive(Debug)]
pub struct Packet59 {}

impl PacketBody for Packet59 {
    const TAG: u8 = 59;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
