use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 67. No information known yet.
#[derive(Debug)]
pub struct Packet67 {}

impl PacketBody for Packet67 {
    const TAG: u8 = 67;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
