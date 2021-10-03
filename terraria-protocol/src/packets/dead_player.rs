use crate::serde::packet_struct;

packet_struct! {
    /// A certain player has died.
    ///
    /// Direction: Server -> Client.
    pub struct DeadPlayer {
        const TAG = 135;

        pub player_id: u8,
    }
}
