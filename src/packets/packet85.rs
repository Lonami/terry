use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 85. No information known yet.
#[derive(Debug)]
pub struct Packet85 {}

impl PacketBody for Packet85 {
    const TAG: u8 = 85;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
