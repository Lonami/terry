use crate::packets::packet_struct;

packet_struct! {
    /// Player dodging.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerDodge {
        const TAG = 62;

        pub player_id: u8,
        /// 1 = Ninja Dodge 2 = Shadow Dodge
        pub flag: u8,
    }
}
