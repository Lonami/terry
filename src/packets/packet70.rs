use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 70. No information known yet.
#[derive(Debug)]
pub struct Packet70 {}

impl PacketBody for Packet70 {
    const TAG: u8 = 70;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
