use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 27. No information known yet.
#[derive(Debug)]
pub struct Packet27 {}

impl PacketBody for Packet27 {
    const TAG: u8 = 27;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
