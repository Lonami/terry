use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Add NPC Buff.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct AddNPCBuff {
    pub npc_id: i16,
    pub buff: u16,
    pub time: i16,
}

impl PacketBody for AddNPCBuff {
    const TAG: u8 = 53;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.buff);
        cursor.write(&self.time);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            buff: cursor.read(),
            time: cursor.read(),
        }
    }
}
