use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 56. No information known yet.
#[derive(Debug)]
pub struct Packet56 {}

impl PacketBody for Packet56 {
    const TAG: u8 = 56;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
