use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 139. No information known yet.
#[derive(Debug)]
pub struct Packet139 {}

impl PacketBody for Packet139 {
    const TAG: u8 = 139;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
