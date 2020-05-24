use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 110. No information known yet.
#[derive(Debug)]
pub struct Packet110 {}

impl PacketBody for Packet110 {
    const TAG: u8 = 110;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
