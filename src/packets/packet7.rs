use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 7 has a lot of information with regards to the world.
#[derive(Debug)]
pub struct Packet7 {}

impl PacketBody for Packet7 {
    const TAG: u8 = 7;

    fn write_body(&self, _cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
