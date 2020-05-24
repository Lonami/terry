use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 131. No information known yet.
#[derive(Debug)]
pub struct Packet131 {}

impl PacketBody for Packet131 {
    const TAG: u8 = 131;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
