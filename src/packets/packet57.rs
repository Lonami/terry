use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet 57, used during login.
#[derive(Debug)]
pub struct Packet57 {
    pub good: u8,
    pub evil: u8,
    pub blood: u8,
}

impl PacketBody for Packet57 {
    const TAG: u8 = 57;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.good);
        cursor.write(&self.evil);
        cursor.write(&self.blood);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            good: cursor.read(),
            evil: cursor.read(),
            blood: cursor.read(),
        }
    }
}
