use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 62. No information known yet.
#[derive(Debug)]
pub struct Packet62 {}

impl PacketBody for Packet62 {
    const TAG: u8 = 62;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
