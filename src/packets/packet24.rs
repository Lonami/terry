use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 24. No information known yet.
#[derive(Debug)]
pub struct Packet24 {}

impl PacketBody for Packet24 {
    const TAG: u8 = 24;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
