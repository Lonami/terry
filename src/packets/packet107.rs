use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 107. No information known yet.
#[derive(Debug)]
pub struct Packet107 {}

impl PacketBody for Packet107 {
    const TAG: u8 = 107;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
