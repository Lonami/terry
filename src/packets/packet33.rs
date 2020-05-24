use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 33. No information known yet.
#[derive(Debug)]
pub struct Packet33 {}

impl PacketBody for Packet33 {
    const TAG: u8 = 33;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
