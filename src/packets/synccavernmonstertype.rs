use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// SyncCavernMonsterType.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct SyncCavernMonsterType {
    /// NPC.cavernMonsterType
    pub net_id: u16,
    /// NPC.cavernMonsterType
    pub net_id: u16,
    /// NPC.cavernMonsterType
    pub net_id: u16,
    /// NPC.cavernMonsterType
    pub net_id: u16,
    /// NPC.cavernMonsterType
    pub net_id: u16,
    /// NPC.cavernMonsterType
    pub net_id: u16,
}

impl PacketBody for SyncCavernMonsterType {
    const TAG: u8 = 136;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.net_id);
        cursor.write(&self.net_id);
        cursor.write(&self.net_id);
        cursor.write(&self.net_id);
        cursor.write(&self.net_id);
        cursor.write(&self.net_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            net_id: cursor.read(),
            net_id: cursor.read(),
            net_id: cursor.read(),
            net_id: cursor.read(),
            net_id: cursor.read(),
            net_id: cursor.read(),
        }
    }
}
