use crate::packets::PacketBody;
use crate::SliceCursor;

/// Fish out a NPC.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct FishOutNpc {
    pub x: u16,
    pub y: u16,
    pub npc_id: i16,
}

impl PacketBody for FishOutNpc {
    const TAG: u8 = 130;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.npc_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            npc_id: cursor.read(),
        }
    }
}
