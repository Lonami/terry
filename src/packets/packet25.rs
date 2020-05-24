use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 25. No information known yet.
#[derive(Debug)]
pub struct Packet25 {}

impl PacketBody for Packet25 {
    const TAG: u8 = 25;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
