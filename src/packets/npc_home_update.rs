use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// NPC Home Update.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct NPCHomeUpdate {
    pub npc_id: i16,
    pub home_tile_x: i16,
    pub home_tile_y: i16,
    pub homeless: u8,
}

impl PacketBody for NPCHomeUpdate {
    const TAG: u8 = 60;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.home_tile_x);
        cursor.write(&self.home_tile_y);
        cursor.write(&self.homeless);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            home_tile_x: cursor.read(),
            home_tile_y: cursor.read(),
            homeless: cursor.read(),
        }
    }
}
