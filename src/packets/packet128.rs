use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 128. No information known yet.
#[derive(Debug)]
pub struct Packet128 {}

impl PacketBody for Packet128 {
    const TAG: u8 = 128;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
