use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 126. No information known yet.
#[derive(Debug)]
pub struct Packet126 {}

impl PacketBody for Packet126 {
    const TAG: u8 = 126;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
