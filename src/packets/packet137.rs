use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 137. No information known yet.
#[derive(Debug)]
pub struct Packet137 {}

impl PacketBody for Packet137 {
    const TAG: u8 = 137;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
