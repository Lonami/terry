use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 113. No information known yet.
#[derive(Debug)]
pub struct Packet113 {}

impl PacketBody for Packet113 {
    const TAG: u8 = 113;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
