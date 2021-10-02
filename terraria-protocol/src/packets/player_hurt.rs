use crate::packets::packet_struct;
use crate::structures::PlayerDeathReason;

packet_struct! {
    /// Player hurt (version 2).
    ///
    /// Direction: Client -> Server.
    pub struct PlayerHurt {
        const TAG = 117;

        pub player_id: u8,
        pub player_death_reason: PlayerDeathReason,
        pub damage: i16,
        pub hit_direction: u8,
        /// BitFlags: 1 = Crit, 2 = PvP
        pub flags: u8,
        pub cooldown_counter: i8,
    }
}
