use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 123. No information known yet.
#[derive(Debug)]
pub struct Packet123 {}

impl PacketBody for Packet123 {
    const TAG: u8 = 123;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
