use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Used once during connection to receive the spawn sections, the sections
/// of the position unless it's -1, and synchronizes the portals and sections
/// around them.
///
/// Direction: Client to Server.
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
        Self { spawn_x: cursor.read(), spawn_y: cursor.read() }
    }
}
