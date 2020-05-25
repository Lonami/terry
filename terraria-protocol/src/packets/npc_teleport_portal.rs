use crate::packets::PacketBody;
use crate::structures::Vec2;
use crate::SliceCursor;

/// NPC using a teleport portal.
///
/// Direction: Server <-> Client.
#[derive(Debug)]
pub struct NpcTeleportPortal {
    pub npc_id: u16,
    pub portal_color_index: i16,
    pub pos: Vec2,
    pub vel: Vec2,
}

impl PacketBody for NpcTeleportPortal {
    const TAG: u8 = 100;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.portal_color_index);
        cursor.write(&self.pos);
        cursor.write(&self.vel);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            portal_color_index: cursor.read(),
            pos: cursor.read(),
            vel: cursor.read(),
        }
    }
}
