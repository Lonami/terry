use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 106. No information known yet.
#[derive(Debug)]
pub struct Packet106 {}

impl PacketBody for Packet106 {
    const TAG: u8 = 106;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
