use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 65. No information known yet.
#[derive(Debug)]
pub struct Packet65 {}

impl PacketBody for Packet65 {
    const TAG: u8 = 65;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
