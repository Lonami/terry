use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 64. No information known yet.
#[derive(Debug)]
pub struct Packet64 {}

impl PacketBody for Packet64 {
    const TAG: u8 = 64;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
