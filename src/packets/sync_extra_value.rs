use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Sync an extra value.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SyncExtraValue {
    pub npc_index: i16,
    pub extra_value: i32,
    pub x: i32, /* single */
    pub y: i32, /* single */
}

impl PacketBody for SyncExtraValue {
    const TAG: u8 = 92;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_index);
        cursor.write(&self.extra_value);
        cursor.write(&self.x);
        cursor.write(&self.y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_index: cursor.read(),
            extra_value: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
        }
    }
}
