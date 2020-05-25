use crate::packets::PacketBody;
use crate::SliceCursor;

/// Door Toggle.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct DoorToggle {
    /// 0 = Open Door, 1 = Close Door, 2 = Open Trapdoor, 3 = Close Trapdoor, 4 = Open Tall Gate, 5 = Close Tall Gate
    pub action: u8,
    pub tile_x: i16,
    pub tile_y: i16,
    /// If (Action == 0) then (if (Direction == -1) then OpenToLeft else OpenToRight) if (Action == 2) then (if (Direction == 1) then PlayerIsAboveTrapdoor) if (Action == 3) then (if (Direction == 1) then PlayerIsAboveTrapdoor)
    pub direction: u8,
}

impl PacketBody for DoorToggle {
    const TAG: u8 = 19;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.action);
        cursor.write(&self.tile_x);
        cursor.write(&self.tile_y);
        cursor.write(&self.direction);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            action: cursor.read(),
            tile_x: cursor.read(),
            tile_y: cursor.read(),
            direction: cursor.read(),
        }
    }
}
