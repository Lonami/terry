use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 29. No information known yet.
#[derive(Debug)]
pub struct Packet29 {}

impl PacketBody for Packet29 {
    const TAG: u8 = 29;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
