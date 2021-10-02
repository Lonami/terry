use crate::packets::packet_struct;

packet_struct! {
    /// Update player luck factors.
    ///
    /// Direction: Client <-> Server.
    pub struct UpdatePlayerLuck {
        const TAG = 134;

        pub player_id: u8,
        pub ladybug_luck_time_remaining: i32,
        pub torch_luck: f32,
        pub luck_potion: u8,
        pub hasgardengnomenearby: bool,
    }
}
