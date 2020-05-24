use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 91. No information known yet.
#[derive(Debug)]
pub struct Packet91 {}

impl PacketBody for Packet91 {
    const TAG: u8 = 91;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
