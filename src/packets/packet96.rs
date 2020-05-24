use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 96. No information known yet.
#[derive(Debug)]
pub struct Packet96 {}

impl PacketBody for Packet96 {
    const TAG: u8 = 96;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
