use crate::packets::PacketBody;
use crate::SliceCursor;

/// Request the essential tiles, used once during the initial connection:
///
/// * Server will send the spawn sections.
/// * If spawn coords aren't -1, server will send the sections of the selected position (which is the player's spawnpoint).
/// * Synchronises all portals and sections around them.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct RequestEssentialTiles {
    pub spawn_x: i32,
    pub spawn_y: i32,
}

impl PacketBody for RequestEssentialTiles {
    const TAG: u8 = 8;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.spawn_x);
        cursor.write(&self.spawn_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            spawn_x: cursor.read(),
            spawn_y: cursor.read(),
        }
    }
}
