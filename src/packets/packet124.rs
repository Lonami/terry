use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 124. No information known yet.
#[derive(Debug)]
pub struct Packet124 {}

impl PacketBody for Packet124 {
    const TAG: u8 = 124;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
