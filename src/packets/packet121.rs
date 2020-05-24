use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 121. No information known yet.
#[derive(Debug)]
pub struct Packet121 {}

impl PacketBody for Packet121 {
    const TAG: u8 = 121;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        // TODO
        Self {}
    }
}
