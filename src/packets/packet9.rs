use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 9, used during login.
#[derive(Debug)]
pub struct Packet9 {
    a: u32,
    b: u8,
    c: String,
    d: u16,
}

impl PacketBody for Packet9 {
    const TAG: u8 = 9;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.a);
        cursor.write(&self.b);
        cursor.write(&self.c);
        cursor.write(&self.d);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            a: cursor.read(),
            b: cursor.read(),
            c: cursor.read(),
            d: cursor.read(),
        }
    }
}
