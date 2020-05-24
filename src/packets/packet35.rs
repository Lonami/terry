use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 35. No information known yet.
#[derive(Debug)]
pub struct Packet35 {}

impl PacketBody for Packet35 {
    const TAG: u8 = 35;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
