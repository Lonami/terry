use crate::serde::packet_struct;

packet_struct! {
    /// Countdown for Moon Lord.
    ///
    /// Direction: Server -> Client.
    pub struct MoonLordCountdown {
        const TAG = 103;

        pub countdown: i32,
    }
}
