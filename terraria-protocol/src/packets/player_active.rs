use crate::packets::packet_struct;

packet_struct! {
    /// Player activity.
    ///
    /// Direction: Server -> Client.
    pub struct PlayerActive {
        const TAG = 14;

        pub player_id: u8,
        pub active: bool,
    }
}
