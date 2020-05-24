use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 39. No information known yet.
#[derive(Debug)]
pub struct Packet39 {}

impl PacketBody for Packet39 {
    const TAG: u8 = 39;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
