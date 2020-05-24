use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 90. No information known yet.
#[derive(Debug)]
pub struct Packet90 {}

impl PacketBody for Packet90 {
    const TAG: u8 = 90;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
