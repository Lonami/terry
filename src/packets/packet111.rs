use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 111. No information known yet.
#[derive(Debug)]
pub struct Packet111 {}

impl PacketBody for Packet111 {
    const TAG: u8 = 111;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
