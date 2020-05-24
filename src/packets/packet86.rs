use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 86. No information known yet.
#[derive(Debug)]
pub struct Packet86 {}

impl PacketBody for Packet86 {
    const TAG: u8 = 86;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
