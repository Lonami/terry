use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 112. No information known yet.
#[derive(Debug)]
pub struct Packet112 {}

impl PacketBody for Packet112 {
    const TAG: u8 = 112;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
