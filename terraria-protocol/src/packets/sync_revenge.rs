use crate::packets::PacketBody;
use crate::structures::Vec2;
use crate::SliceCursor;

/// Sync the revenge marker.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SyncRevenge {
    pub unique_id: i32,
    pub pos: Vec2,
    pub npc_id: i32,
    pub npc_hp_percent: f32,
    pub npc_type: i32,
    pub npc_ai: i32,
    pub coin_value: i32,
    pub base_value: f32,
    pub spawnedfromstatue: bool,
}

impl PacketBody for SyncRevenge {
    const TAG: u8 = 126;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.unique_id);
        cursor.write(&self.pos);
        cursor.write(&self.npc_id);
        cursor.write(&self.npc_hp_percent);
        cursor.write(&self.npc_type);
        cursor.write(&self.npc_ai);
        cursor.write(&self.coin_value);
        cursor.write(&self.base_value);
        cursor.write(&self.spawnedfromstatue);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            unique_id: cursor.read(),
            pos: cursor.read(),
            npc_id: cursor.read(),
            npc_hp_percent: cursor.read(),
            npc_type: cursor.read(),
            npc_ai: cursor.read(),
            coin_value: cursor.read(),
            base_value: cursor.read(),
            spawnedfromstatue: cursor.read(),
        }
    }
}
