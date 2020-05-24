use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 109. No information known yet.
#[derive(Debug)]
pub struct Packet109 {}

impl PacketBody for Packet109 {
    const TAG: u8 = 109;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
