use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 47. No information known yet.
#[derive(Debug)]
pub struct Packet47 {}

impl PacketBody for Packet47 {
    const TAG: u8 = 47;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
