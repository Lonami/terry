use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 48. No information known yet.
#[derive(Debug)]
pub struct Packet48 {}

impl PacketBody for Packet48 {
    const TAG: u8 = 48;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
