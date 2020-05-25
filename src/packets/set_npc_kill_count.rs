use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Set NPC Kill Count.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SetNPCKillCount {
    pub npc_type: i16,
    pub kill_count: i32,
}

impl PacketBody for SetNPCKillCount {
    const TAG: u8 = 83;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_type);
        cursor.write(&self.kill_count);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_type: cursor.read(),
            kill_count: cursor.read(),
        }
    }
}
