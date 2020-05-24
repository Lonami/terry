use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 136. No information known yet.
#[derive(Debug)]
pub struct Packet136 {}

impl PacketBody for Packet136 {
    const TAG: u8 = 136;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
