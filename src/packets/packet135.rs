use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 135. No information known yet.
#[derive(Debug)]
pub struct Packet135 {}

impl PacketBody for Packet135 {
    const TAG: u8 = 135;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
