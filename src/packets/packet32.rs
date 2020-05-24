use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 32. No information known yet.
#[derive(Debug)]
pub struct Packet32 {}

impl PacketBody for Packet32 {
    const TAG: u8 = 32;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
