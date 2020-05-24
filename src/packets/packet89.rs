use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 89. No information known yet.
#[derive(Debug)]
pub struct Packet89 {}

impl PacketBody for Packet89 {
    const TAG: u8 = 89;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
