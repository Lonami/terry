use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 132. No information known yet.
#[derive(Debug)]
pub struct Packet132 {}

impl PacketBody for Packet132 {
    const TAG: u8 = 132;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
