use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 93. No information known yet.
#[derive(Debug)]
pub struct Packet93 {}

impl PacketBody for Packet93 {
    const TAG: u8 = 93;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
