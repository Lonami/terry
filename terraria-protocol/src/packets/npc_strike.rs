use crate::packets::PacketBody;
use crate::SliceCursor;

/// NPC attacks.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct NpcStrike {
    pub npc_id: i16,
    /// -1 = Kill
    pub damage: i16,
    pub knockback: i32, /* single */
    pub hit_direction: u8,
    pub crit: bool,
}

impl PacketBody for NpcStrike {
    const TAG: u8 = 28;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.damage);
        cursor.write(&self.knockback);
        cursor.write(&self.hit_direction);
        cursor.write(&self.crit);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            damage: cursor.read(),
            knockback: cursor.read(),
            hit_direction: cursor.read(),
            crit: cursor.read(),
        }
    }
}
