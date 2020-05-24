use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 104. No information known yet.
#[derive(Debug)]
pub struct Packet104 {}

impl PacketBody for Packet104 {
    const TAG: u8 = 104;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
