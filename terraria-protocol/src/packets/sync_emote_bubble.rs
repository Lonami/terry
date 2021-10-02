use crate::packets::packet_struct;

packet_struct! {
    /// Sync an emote bubble.
    ///
    /// Direction: Server -> Client.
    pub struct SyncEmoteBubble {
        const TAG = 91;

        pub emote_id: i32,
        pub anchor_type: u8,
        /// Only sent if AnchorType != 255
        pub player_id: u16,
        /// Only sent if AnchorType != 255
        pub emote_lifetime: u16,
        /// Only sent if AnchorType != 255
        pub emote: u8,
        /// Only sent if AnchorType != 255 and Emote &lt; 0
        pub emote_metadata: i16,
    }
}
