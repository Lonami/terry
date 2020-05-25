use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Strike NPCwith Held Item.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct StrikeNPCwithHeldItem {
    pub npc_id: i16,
    pub player_id: u8,
}

impl PacketBody for StrikeNPCwithHeldItem {
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
