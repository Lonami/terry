use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Strike a NPC with the held item.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct StrikeNpc {
    pub npc_id: i16,
    pub player_id: u8,
}

impl PacketBody for StrikeNpc {
    const TAG: u8 = 24;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.player_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            player_id: cursor.read(),
        }
    }
}
