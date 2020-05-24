use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 74. No information known yet.
#[derive(Debug)]
pub struct Packet74 {}

impl PacketBody for Packet74 {
    const TAG: u8 = 74;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
