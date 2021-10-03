use crate::packets::packet_struct;

packet_struct! {
    /// In theory "never sent".
    ///
    /// Direction: ???.
    pub struct Null {
        const TAG = 15;

        pub id: i16,
        /// "Terraria" + Main.curRelease
        pub version: String,
    }
}
