use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 80. No information known yet.
#[derive(Debug)]
pub struct Packet80 {}

impl PacketBody for Packet80 {
    const TAG: u8 = 80;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
