use crate::packets::packet_struct;
use crate::structures::Vec2;

packet_struct! {
    /// Teleport a player through a portal.
    ///
    /// Direction: Server <-> Client.
    pub struct PlayerTeleportPortal {
        const TAG = 96;

        pub player_id: u8,
        pub portal_color_index: i16,
        pub pos: Vec2,
        pub vel: Vec2,
    }
}
