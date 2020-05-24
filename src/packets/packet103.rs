use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 103. No information known yet.
#[derive(Debug)]
pub struct Packet103 {}

impl PacketBody for Packet103 {
    const TAG: u8 = 103;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
