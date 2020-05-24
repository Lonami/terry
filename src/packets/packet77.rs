use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 77. No information known yet.
#[derive(Debug)]
pub struct Packet77 {}

impl PacketBody for Packet77 {
    const TAG: u8 = 77;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
