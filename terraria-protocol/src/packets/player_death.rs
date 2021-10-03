use crate::serde::packet_struct;
use crate::structures::PlayerDeathReason;

packet_struct! {
    /// Player death (version 2).
    ///
    /// Direction: Client -> Server.
    pub struct PlayerDeath {
        const TAG = 118;

        pub player_id: u8,
        pub player_death_reason: PlayerDeathReason,
        pub damage: i16,
        pub hit_direction: u8,
        pub pvp: bool,
    }
}
