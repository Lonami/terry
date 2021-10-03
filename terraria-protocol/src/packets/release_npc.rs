use crate::serde::packet_struct;

packet_struct! {
    /// Release a NPC.
    ///
    /// Direction: Client -> Server.
    pub struct ReleaseNpc {
        const TAG = 71;

        pub x: i32,
        pub y: i32,
        pub npc_type: i16,
        /// Sent to NPC AI[2].
        pub style: u8,
    }
}
