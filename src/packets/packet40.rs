use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 40. No information known yet.
#[derive(Debug)]
pub struct Packet40 {}

impl PacketBody for Packet40 {
    const TAG: u8 = 40;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
