use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Update NPC Name.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdateNPCName {
    pub npc_id: i16,
    /// Only if client is receiving packet
    pub name: String,
    /// Only if client is receiving packet
    pub townnpcvariationindex: i32,
}

impl PacketBody for UpdateNPCName {
    const TAG: u8 = 56;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.name);
        cursor.write(&self.townnpcvariationindex);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            name: cursor.read(),
            townnpcvariationindex: cursor.read(),
        }
    }
}
