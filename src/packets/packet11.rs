use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 11, used during login.
#[derive(Debug)]
pub struct Packet11 {
    pub a: i16,
    pub b: i16,
    pub c: i16,
    pub d: i16,
}

impl PacketBody for Packet11 {
    const TAG: u8 = 11;

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
