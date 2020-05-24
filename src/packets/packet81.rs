use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 81. No information known yet.
#[derive(Debug)]
pub struct Packet81 {}

impl PacketBody for Packet81 {
    const TAG: u8 = 81;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
