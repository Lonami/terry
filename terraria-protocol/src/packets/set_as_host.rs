use crate::packets::packet_struct;

packet_struct! {
    /// Set that some player counts as the host for gameplay.
    ///
    /// Direction: Server -> Client.
    pub struct SetAsHost {
        const TAG = 139;

        pub player_id: u8,
        pub countsashost: bool,
    }
}
