use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 100. No information known yet.
#[derive(Debug)]
pub struct Packet100 {}

impl PacketBody for Packet100 {
    const TAG: u8 = 100;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
