use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 129. No information known yet.
#[derive(Debug)]
pub struct Packet129 {}

impl PacketBody for Packet129 {
    const TAG: u8 = 129;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
