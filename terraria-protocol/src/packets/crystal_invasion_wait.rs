use crate::packets::packet_struct;

packet_struct! {
    /// Wait time until the next wave of the Eternia Crystal Invasion.
    ///
    /// Direction: Server -> Client.
    pub struct CrystalInvasionWait {
        const TAG = 116;

        /// 1800 (30s) between waves, 30 (5s) when starting
        pub time_until_next_wave: i32,
    }
}
