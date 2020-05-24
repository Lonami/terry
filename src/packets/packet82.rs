use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 82, used during login. Seems to be chat messages.
#[derive(Debug)]
pub struct Packet82 {
    pub a: u8,
    pub b: u16,
}

impl PacketBody for Packet82 {
    const TAG: u8 = 82;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.a);
        cursor.write(&self.b);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            a: cursor.read(),
            b: cursor.read(),
        }
    }
}
