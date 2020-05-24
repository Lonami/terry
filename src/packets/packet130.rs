use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 130. No information known yet.
#[derive(Debug)]
pub struct Packet130 {}

impl PacketBody for Packet130 {
    const TAG: u8 = 130;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
