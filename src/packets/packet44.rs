use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 44. No information known yet.
#[derive(Debug)]
pub struct Packet44 {}

impl PacketBody for Packet44 {
    const TAG: u8 = 44;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
