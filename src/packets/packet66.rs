use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 66. No information known yet.
#[derive(Debug)]
pub struct Packet66 {}

impl PacketBody for Packet66 {
    const TAG: u8 = 66;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
