use crate::serde::packet_struct;

packet_struct! {
    /// Set player stealth.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct SetPlayerStealth {
        const TAG = 84;

        pub player: u8,
        pub stealth: f32,
    }
}
