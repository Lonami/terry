use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 26. No information known yet.
#[derive(Debug)]
pub struct Packet26 {}

impl PacketBody for Packet26 {
    const TAG: u8 = 26;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
