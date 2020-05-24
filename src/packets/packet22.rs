use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 22, used during login.
#[derive(Debug)]
pub struct Packet22 {
    pub a: u8,
    pub b: u8,
}

impl PacketBody for Packet22 {
    const TAG: u8 = 22;

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
