use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 14. No information known yet.
#[derive(Debug)]
pub struct Packet14 {}

impl PacketBody for Packet14 {
    const TAG: u8 = 14;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
