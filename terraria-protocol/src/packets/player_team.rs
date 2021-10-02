use crate::packets::packet_struct;

packet_struct! {
    /// Set the player's team.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerTeam {
        const TAG = 45;

        pub player_id: u8,
        pub team: u8,
    }
}
