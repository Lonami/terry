use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// NPC using a teleport portal.
///
/// Direction: Server <-> Client.
#[derive(Debug)]
pub struct NpcTeleportPortal {
    pub npc_id: u16,
    pub portal_color_index: i16,
    pub new_position_x: i32 /* single */ ,
    pub new_position_y: i32 /* single */ ,
    pub velocity_x: i32 /* single */ ,
    pub velocity_y: i32 /* single */ ,
}

impl PacketBody for NpcTeleportPortal {
    const TAG: u8 = 100;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.portal_color_index);
        cursor.write(&self.new_position_x);
        cursor.write(&self.new_position_y);
        cursor.write(&self.velocity_x);
        cursor.write(&self.velocity_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            portal_color_index: cursor.read(),
            new_position_x: cursor.read(),
            new_position_y: cursor.read(),
            velocity_x: cursor.read(),
            velocity_y: cursor.read(),
        }
    }
}
