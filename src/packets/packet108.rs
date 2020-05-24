use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 108. No information known yet.
#[derive(Debug)]
pub struct Packet108 {}

impl PacketBody for Packet108 {
    const TAG: u8 = 108;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
