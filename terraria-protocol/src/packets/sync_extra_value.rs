use crate::packets::PacketBody;
use crate::SliceCursor;
use crate::structures::Vec2;

/// Sync an extra value.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SyncExtraValue {
    pub npc_index: i16,
    pub extra_value: i32,
    pub pos: Vec2,
}

impl PacketBody for SyncExtraValue {
    const TAG: u8 = 92;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_index);
        cursor.write(&self.extra_value);
        cursor.write(&self.pos);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_index: cursor.read(),
            extra_value: cursor.read(),
            pos: cursor.read(),
        }
    }
}
