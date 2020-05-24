use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 46. No information known yet.
#[derive(Debug)]
pub struct Packet46 {}

impl PacketBody for Packet46 {
    const TAG: u8 = 46;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
