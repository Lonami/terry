use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Catch NPC.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct CatchNPC {
    pub npc_id: i16,
    pub player_id: u8,
}

impl PacketBody for CatchNPC {
    const TAG: u8 = 70;

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
