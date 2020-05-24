use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 3, used during login.
#[derive(Debug)]
pub struct Packet3 {
    pub a: u8,
}

impl PacketBody for Packet3 {
    const TAG: u8 = 3;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.a);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self { a: cursor.read() }
    }
}
