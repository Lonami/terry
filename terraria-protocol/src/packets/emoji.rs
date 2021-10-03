use crate::serde::packet_struct;

packet_struct! {
    /// Emoji.
    ///
    /// Direction: Client <-> Server.
    pub struct Emoji {
        const TAG = 120;

        pub player_id: u8,
        pub emoticon: u8,
    }
}
