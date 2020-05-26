use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update NPC name.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdateNpcName {
    pub npc_id: i16,
    // TODO we don't know if we're receiving or not!
    // Only if client is receiving packet
    //pub name: String,
    // Only if client is receiving packet
    //pub townnpcvariationindex: i32,
}

impl PacketBody for UpdateNpcName {
    const TAG: u8 = 56;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
        }
    }
}
