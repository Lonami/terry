use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 78. No information known yet.
#[derive(Debug)]
pub struct Packet78 {}

impl PacketBody for Packet78 {
    const TAG: u8 = 78;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
