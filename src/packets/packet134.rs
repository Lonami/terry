use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 134. No information known yet.
#[derive(Debug)]
pub struct Packet134 {}

impl PacketBody for Packet134 {
    const TAG: u8 = 134;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
