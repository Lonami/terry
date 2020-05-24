use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 30. No information known yet.
#[derive(Debug)]
pub struct Packet30 {}

impl PacketBody for Packet30 {
    const TAG: u8 = 30;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
