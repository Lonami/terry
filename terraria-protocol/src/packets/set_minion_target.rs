use crate::serde::packet_struct;

packet_struct! {
    /// Update a minion's attack target.
    ///
    /// Direction: Client -> Server.
    pub struct SetMinionTarget {
        const TAG = 115;

        pub player_id: u8,
        pub minion_attack_target: i16,
    }
}
