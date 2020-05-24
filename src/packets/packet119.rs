use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 119. No information known yet.
#[derive(Debug)]
pub struct Packet119 {}

impl PacketBody for Packet119 {
    const TAG: u8 = 119;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
