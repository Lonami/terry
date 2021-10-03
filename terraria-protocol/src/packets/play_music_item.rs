use crate::serde::packet_struct;

packet_struct! {
    /// Play a music item.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayMusicItem {
        const TAG = 58;

        pub player_id: u8,
        pub note: f32,
    }
}
