use crate::packets::PacketBody;
use crate::SliceCursor;

/// Unlock.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct Unlock {
    /// Values: 1 = Chest Unlock, 2 = Door Unlock
    pub ty: u8,
    pub x: i16,
    pub y: i16,
}

impl PacketBody for Unlock {
    const TAG: u8 = 52;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.ty);
        cursor.write(&self.x);
        cursor.write(&self.y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            ty: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
        }
    }
}
