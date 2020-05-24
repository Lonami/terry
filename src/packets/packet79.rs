use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 79. No information known yet.
#[derive(Debug)]
pub struct Packet79 {}

impl PacketBody for Packet79 {
    const TAG: u8 = 79;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
