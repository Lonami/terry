use crate::packets::packet_struct;
use crate::structures::{serializable_bitflags, PlayerDeathReason};

serializable_bitflags! {
    pub struct DamageType: u8 {
        const CRITICAL = 0x01;
        const PVP = 0x02;
    }
}

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
        pub ty: DamageType,
        pub cooldown_counter: i8,
    }
}
