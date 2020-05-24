use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 101. No information known yet.
#[derive(Debug)]
pub struct Packet101 {}

impl PacketBody for Packet101 {
    const TAG: u8 = 101;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
