use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 18. No information known yet.
#[derive(Debug)]
pub struct Packet18 {}

impl PacketBody for Packet18 {
    const TAG: u8 = 18;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
