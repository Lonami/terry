use crate::packets::PacketBody;
use crate::SliceCursor;

/// Add a buff (or debuff) to some NPC for a certain duration.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct AddNpcBuff {
    pub npc_id: i16,
    pub buff: u16,
    pub time: i16,
}

impl PacketBody for AddNpcBuff {
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
