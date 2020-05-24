use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 98. No information known yet.
#[derive(Debug)]
pub struct Packet98 {}

impl PacketBody for Packet98 {
    const TAG: u8 = 98;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
