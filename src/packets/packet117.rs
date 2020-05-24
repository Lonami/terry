use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 117. No information known yet.
#[derive(Debug)]
pub struct Packet117 {}

impl PacketBody for Packet117 {
    const TAG: u8 = 117;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
