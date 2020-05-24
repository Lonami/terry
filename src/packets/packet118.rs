use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 118. No information known yet.
#[derive(Debug)]
pub struct Packet118 {}

impl PacketBody for Packet118 {
    const TAG: u8 = 118;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
