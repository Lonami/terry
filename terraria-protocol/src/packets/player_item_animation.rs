use crate::packets::packet_struct;

packet_struct! {
    /// Player item animation.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerItemAnimation {
        const TAG = 41;

        pub player_id: u8,
        pub item_rotation: f32,
        pub item_animation: i16,
    }
}
