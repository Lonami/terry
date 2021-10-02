use crate::packets::packet_struct;

packet_struct! {
    /// Connect request, sent at the very beginning of the communication.
    ///
    /// Direction: Client -> Server.
    pub struct Connect {
        const TAG = 1;

        /// "Terraria" + Main.curRelease
        pub version: String,
    }
}
