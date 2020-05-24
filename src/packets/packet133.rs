use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 133. No information known yet.
#[derive(Debug)]
pub struct Packet133 {}

impl PacketBody for Packet133 {
    const TAG: u8 = 133;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
