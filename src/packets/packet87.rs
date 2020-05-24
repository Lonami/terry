use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 87. No information known yet.
#[derive(Debug)]
pub struct Packet87 {}

impl PacketBody for Packet87 {
    const TAG: u8 = 87;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
