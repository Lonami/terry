use crate::serde::{packet_struct, serializable_enum};

serializable_enum! {
    pub enum SpawnContext: u8 {
        ReviveFromDeath = 0,
        SpawningIntoWorld = 1,
        RecallFromItem = 2,
    }
}

packet_struct! {
    /// Spawn a player.
    ///
    /// Direction: Client -> Server.
    pub struct SpawnPlayer {
        const TAG = 12;

        pub player_id: u8,
        pub spawn_x: i16,
        pub spawn_y: i16,
        /// If > 0, then player is still dead
        pub respawn_time_remaining: i32,
        pub player_spawn_context: SpawnContext,
    }
}
