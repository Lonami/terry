use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 115. No information known yet.
#[derive(Debug)]
pub struct Packet115 {}

impl PacketBody for Packet115 {
    const TAG: u8 = 115;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
