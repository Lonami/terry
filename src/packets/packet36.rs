use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 36. No information known yet.
#[derive(Debug)]
pub struct Packet36 {}

impl PacketBody for Packet36 {
    const TAG: u8 = 36;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
