use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 102. No information known yet.
#[derive(Debug)]
pub struct Packet102 {}

impl PacketBody for Packet102 {
    const TAG: u8 = 102;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
