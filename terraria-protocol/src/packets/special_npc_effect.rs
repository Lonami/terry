use crate::serde::{packet_struct, serializable_enum};

serializable_enum! {
    pub enum SpecialEffect: u8 {
        SpawnSkeletron = 1,
        CauseSoundAtPlayer = 2,
        StartSundialing = 3,
        BigMimicSpawnSmoke = 4,
    }
}

packet_struct! {
    /// Special NPC Effect.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct SpecialNpcEffect {
        const TAG = 51;

        pub player_id: u8,
        pub ty: SpecialEffect,
    }
}
