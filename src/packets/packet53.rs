use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 53. No information known yet.
#[derive(Debug)]
pub struct Packet53 {}

impl PacketBody for Packet53 {
    const TAG: u8 = 53;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
