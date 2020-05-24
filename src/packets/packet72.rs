use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 72. No information known yet.
#[derive(Debug)]
pub struct Packet72 {}

impl PacketBody for Packet72 {
    const TAG: u8 = 72;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
