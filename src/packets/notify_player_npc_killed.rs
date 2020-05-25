use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Notify Player NPC Killed.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct NotifyPlayerNPCKilled {
    pub npc_id: i16,
}

impl PacketBody for NotifyPlayerNPCKilled {
    const TAG: u8 = 97;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
        }
    }
}
