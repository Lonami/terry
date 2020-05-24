use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 125. No information known yet.
#[derive(Debug)]
pub struct Packet125 {}

impl PacketBody for Packet125 {
    const TAG: u8 = 125;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
