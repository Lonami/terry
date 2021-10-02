use crate::packets::packet_struct;

packet_struct! {
    /// Toggle Player-Versus-Player.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct TogglePvp {
        const TAG = 30;

        pub player_id: u8,
        pub pvp_enabled: bool,
    }
}
