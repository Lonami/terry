use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 45. No information known yet.
#[derive(Debug)]
pub struct Packet45 {}

impl PacketBody for Packet45 {
    const TAG: u8 = 45;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
