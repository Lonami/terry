use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 31. No information known yet.
#[derive(Debug)]
pub struct Packet31 {}

impl PacketBody for Packet31 {
    const TAG: u8 = 31;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
