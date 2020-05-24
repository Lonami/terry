use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 76. No information known yet.
#[derive(Debug)]
pub struct Packet76 {}

impl PacketBody for Packet76 {
    const TAG: u8 = 76;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
