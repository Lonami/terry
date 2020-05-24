use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 17. No information known yet.
#[derive(Debug)]
pub struct Packet17 {}

impl PacketBody for Packet17 {
    const TAG: u8 = 17;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
