use crate::packets::packet_struct;
use crate::structures::Vec2;

packet_struct! {
    /// No description known yet.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerNpcTeleport {
        const TAG = 65;

        /// BitFlags: 0 = Player Teleport (Neither 1 or 2), 1 = NPC Teleport, 2 = Player Teleport to Other Player, 4 = GetPositionFromTarget, 8 = HasExtraInfo
        pub flags: u8,
        pub target_id: i16,
        pub pos: Vec2,
        pub style: u8,
        /// Only sent if HasExtraInfo flag is true
        pub extrainfo: i32,
    }
}
