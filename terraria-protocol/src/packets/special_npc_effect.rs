use crate::packets::packet_struct;

packet_struct! {
    /// Special NPC Effect.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct SpecialNpcEffect {
        const TAG = 51;

        pub player_id: u8,
        /// Values: 1 = Spawn Skeletron, 2 = Cause sound at player, 3 = Start Sundialing (Only works if server is receiving), 4 = BigMimcSpawnSmoke
        pub ty: u8,
    }
}
