use crate::serde::packet_struct;

packet_struct! {
    /// No description known yet.
    ///
    /// Direction: ???.
    pub struct Packet15 {
        const TAG = 15;

        pub id: i16,
        /// "Terraria" + Main.curRelease
        pub version: String,
    }
}
