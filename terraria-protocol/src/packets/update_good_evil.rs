use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update Good Evil.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct UpdateGoodEvil {
    pub good: u8,
    pub evil: u8,
    pub crimson: u8,
}

impl PacketBody for UpdateGoodEvil {
    const TAG: u8 = 57;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.good);
        cursor.write(&self.evil);
        cursor.write(&self.crimson);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            good: cursor.read(),
            evil: cursor.read(),
            crimson: cursor.read(),
        }
    }
}
