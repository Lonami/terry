use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 75. No information known yet.
#[derive(Debug)]
pub struct Packet75 {}

impl PacketBody for Packet75 {
    const TAG: u8 = 75;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
