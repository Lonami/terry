use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 105. No information known yet.
#[derive(Debug)]
pub struct Packet105 {}

impl PacketBody for Packet105 {
    const TAG: u8 = 105;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
