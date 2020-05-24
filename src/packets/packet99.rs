use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 99. No information known yet.
#[derive(Debug)]
pub struct Packet99 {}

impl PacketBody for Packet99 {
    const TAG: u8 = 99;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
