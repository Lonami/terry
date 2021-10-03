use crate::packets::packet_struct;
use crate::structures::Vec2;

packet_struct! {
    /// NPC using a teleport portal.
    ///
    /// Direction: Server <-> Client.
    pub struct NpcTeleportPortal {
        const TAG = 100;

        pub npc_id: u16,
        pub portal_color_index: u16,
        pub pos: Vec2,
        pub vel: Vec2,
    }
}
