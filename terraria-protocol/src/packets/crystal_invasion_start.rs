use crate::packets::packet_struct;

packet_struct! {
    /// Start the Invasion of the Eternia Crystal.
    ///
    /// Direction: Client -> Server.
    pub struct CrystalInvasionStart {
        const TAG = 113;

        pub x: i16,
        pub y: i16,
    }
}
