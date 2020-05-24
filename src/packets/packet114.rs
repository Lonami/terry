use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 114. No information known yet.
#[derive(Debug)]
pub struct Packet114 {}

impl PacketBody for Packet114 {
    const TAG: u8 = 114;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
