use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 63. No information known yet.
#[derive(Debug)]
pub struct Packet63 {}

impl PacketBody for Packet63 {
    const TAG: u8 = 63;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
