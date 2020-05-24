use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 84. No information known yet.
#[derive(Debug)]
pub struct Packet84 {}

impl PacketBody for Packet84 {
    const TAG: u8 = 84;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
