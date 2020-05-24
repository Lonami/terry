use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 38. No information known yet.
#[derive(Debug)]
pub struct Packet38 {}

impl PacketBody for Packet38 {
    const TAG: u8 = 38;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
