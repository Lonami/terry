use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 55. No information known yet.
#[derive(Debug)]
pub struct Packet55 {}

impl PacketBody for Packet55 {
    const TAG: u8 = 55;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
