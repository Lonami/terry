use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 34. No information known yet.
#[derive(Debug)]
pub struct Packet34 {}

impl PacketBody for Packet34 {
    const TAG: u8 = 34;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
