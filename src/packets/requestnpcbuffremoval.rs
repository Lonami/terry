use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// RequestNPCBuffRemoval.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct RequestNPCBuffRemoval {
    pub npc_id: i16,
    pub buff_id: u16,
}

impl PacketBody for RequestNPCBuffRemoval {
    const TAG: u8 = 137;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.buff_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            buff_id: cursor.read(),
        }
    }
}
