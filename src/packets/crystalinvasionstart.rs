use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// CrystalInvasionStart.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct CrystalInvasionStart {
    pub x: i16,
    pub y: i16,
}

impl PacketBody for CrystalInvasionStart {
    const TAG: u8 = 113;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
        }
    }
}
