use crate::packets::packet_struct;

packet_struct! {
    /// Teleportation potion.
    ///
    /// Direction: Server <-> Client.
    pub struct TeleportationPotion {
        const TAG = 73;

        /// 0 = Teleportation Potion, 1 = Magic Conch, 2 = Demon Conch
        pub ty: u8,
    }
}
