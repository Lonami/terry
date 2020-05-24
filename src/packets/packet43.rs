use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 43. No information known yet.
#[derive(Debug)]
pub struct Packet43 {}

impl PacketBody for Packet43 {
    const TAG: u8 = 43;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
