use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 88. No information known yet.
#[derive(Debug)]
pub struct Packet88 {}

impl PacketBody for Packet88 {
    const TAG: u8 = 88;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
