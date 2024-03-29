use crate::serde::packet_struct;

packet_struct! {
    /// Set the user's slot.
    ///
    /// Direction: Server -> Client.
    pub struct SetUserSlot {
        const TAG = 3;

        pub player_id: u8,
        pub server_flag: bool,
    }
}
