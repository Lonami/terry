use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 51. No information known yet.
#[derive(Debug)]
pub struct Packet51 {}

impl PacketBody for Packet51 {
    const TAG: u8 = 51;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
