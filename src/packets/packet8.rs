use crate::packets::Packet;
use crate::serialization::SliceCursor;

/// Packet 8, used during login. Seems to always be -1.
pub struct Packet8 {
    pub n: i32,
}

impl Packet for Packet8 {
    const TAG: u8 = 8;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.n);
    }

    fn from_body(&self, cursor: &mut SliceCursor) -> Self {
        Self { n: cursor.read() }
    }
}
