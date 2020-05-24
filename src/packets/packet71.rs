use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 71. No information known yet.
#[derive(Debug)]
pub struct Packet71 {}

impl PacketBody for Packet71 {
    const TAG: u8 = 71;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
