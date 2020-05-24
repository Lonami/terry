use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 19. No information known yet.
#[derive(Debug)]
pub struct Packet19 {}

impl PacketBody for Packet19 {
    const TAG: u8 = 19;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
