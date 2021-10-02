use crate::packets::packet_struct;

packet_struct! {
    /// Update Shield Strengths.
    ///
    /// Direction: Server -> Client.
    pub struct UpdateShieldStrengths {
        const TAG = 101;

        pub solar_tower_shield_strength: u16,
        pub vortex_tower_shield_strength: u16,
        pub nebula_tower_shield_strength: u16,
        pub stardust_tower_shield_strength: u16,
    }
}
