use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 15. No information known yet.
#[derive(Debug)]
pub struct Packet15 {}

impl PacketBody for Packet15 {
    const TAG: u8 = 15;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
