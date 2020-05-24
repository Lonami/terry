use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 2. No information known yet.
#[derive(Debug)]
pub struct Packet2 {}

impl PacketBody for Packet2 {
    const TAG: u8 = 2;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
