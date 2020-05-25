use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Release NPC.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ReleaseNPC {
    pub x: i32,
    pub y: i32,
    pub npc_type: i16,
    /// Sent to NPC AI
    pub style: u8,
}

impl PacketBody for ReleaseNPC {
    const TAG: u8 = 71;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.npc_type);
        cursor.write(&self.style);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            npc_type: cursor.read(),
            style: cursor.read(),
        }
    }
}
