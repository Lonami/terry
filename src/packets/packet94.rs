use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 94. No information known yet.
#[derive(Debug)]
pub struct Packet94 {}

impl PacketBody for Packet94 {
    const TAG: u8 = 94;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
