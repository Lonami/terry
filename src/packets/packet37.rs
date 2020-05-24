use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 37. No information known yet.
#[derive(Debug)]
pub struct Packet37 {}

impl PacketBody for Packet37 {
    const TAG: u8 = 37;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
