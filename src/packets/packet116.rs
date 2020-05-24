use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 116. No information known yet.
#[derive(Debug)]
pub struct Packet116 {}

impl PacketBody for Packet116 {
    const TAG: u8 = 116;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
