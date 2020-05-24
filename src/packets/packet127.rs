use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 127. No information known yet.
#[derive(Debug)]
pub struct Packet127 {}

impl PacketBody for Packet127 {
    const TAG: u8 = 127;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
