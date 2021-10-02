use crate::packets::packet_struct;
use crate::structures::Vec2;

packet_struct! {
    /// Play a legacy sound.
    ///
    /// Direction: Server -> Client.
    pub struct PlayLegacySound {
        const TAG = 132;

        pub pos: Vec2,
        pub sound_id: u16,
        /// BitFlags: 1 = Style, 2 = Volume Scale, 3 = Pitch Offset
        pub sound_flags: u8,
    }
}
