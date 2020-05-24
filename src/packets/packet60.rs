use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 60. No information known yet.
#[derive(Debug)]
pub struct Packet60 {}

impl PacketBody for Packet60 {
    const TAG: u8 = 60;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
