use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 69. No information known yet.
#[derive(Debug)]
pub struct Packet69 {}

impl PacketBody for Packet69 {
    const TAG: u8 = 69;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
