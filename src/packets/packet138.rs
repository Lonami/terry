use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 138. No information known yet.
#[derive(Debug)]
pub struct Packet138 {}

impl PacketBody for Packet138 {
    const TAG: u8 = 138;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
