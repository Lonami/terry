use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 20. No information known yet.
#[derive(Debug)]
pub struct Packet20 {}

impl PacketBody for Packet20 {
    const TAG: u8 = 20;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
