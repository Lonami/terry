use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 122. No information known yet.
#[derive(Debug)]
pub struct Packet122 {}

impl PacketBody for Packet122 {
    const TAG: u8 = 122;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
