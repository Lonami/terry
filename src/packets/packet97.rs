use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 97. No information known yet.
#[derive(Debug)]
pub struct Packet97 {}

impl PacketBody for Packet97 {
    const TAG: u8 = 97;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
