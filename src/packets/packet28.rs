use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 28. No information known yet.
#[derive(Debug)]
pub struct Packet28 {}

impl PacketBody for Packet28 {
    const TAG: u8 = 28;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
