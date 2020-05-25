use crate::packets::PacketBody;
use crate::SliceCursor;

/// Set liquid.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SetLiquid {
    pub x: i16,
    pub y: i16,
    pub liquid: u8,
    pub liquid_type: u8,
}

impl PacketBody for SetLiquid {
    const TAG: u8 = 48;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.liquid);
        cursor.write(&self.liquid_type);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            liquid: cursor.read(),
            liquid_type: cursor.read(),
        }
    }
}
