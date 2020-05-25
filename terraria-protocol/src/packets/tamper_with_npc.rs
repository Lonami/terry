use crate::packets::PacketBody;
use crate::SliceCursor;

/// Tamper with a NPC.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct TamperWithNpc {
    pub npc_id: u16,
    pub setnpcimmunity: u8,
    /// Only sent if SetNPCImmunity flag is true
    pub immunity_time: i32,
    /// Set to -1 for immunity from all players
    pub immunity_player_id: i16,
}

impl PacketBody for TamperWithNpc {
    const TAG: u8 = 131;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.setnpcimmunity);
        cursor.write(&self.immunity_time);
        cursor.write(&self.immunity_player_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            setnpcimmunity: cursor.read(),
            immunity_time: cursor.read(),
            immunity_player_id: cursor.read(),
        }
    }
}
