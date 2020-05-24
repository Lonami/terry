use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 41. No information known yet.
#[derive(Debug)]
pub struct Packet41 {}

impl PacketBody for Packet41 {
    const TAG: u8 = 41;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
