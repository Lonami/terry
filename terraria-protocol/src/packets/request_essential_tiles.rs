use crate::serde::packet_struct;

packet_struct! {
    /// Request the essential tiles, used once during the initial connection:
    ///
    /// * Server will send the spawn sections.
    /// * If spawn coords aren't -1, server will send the sections of the selected position (which is the player's spawnpoint).
    /// * Synchronises all portals and sections around them.
    ///
    /// Direction: Client -> Server.
    pub struct RequestEssentialTiles {
        const TAG = 8;

        pub spawn_x: i32,
        pub spawn_y: i32,
    }
}
