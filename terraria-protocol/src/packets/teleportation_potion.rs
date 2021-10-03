use crate::packets::packet_struct;
use crate::structures::serializable_enum;

serializable_enum! {
    pub enum TeleportationMethod: u8 {
        Potion = 0,
        MagicConch = 1,
        DemonConch = 2,
    }
}

packet_struct! {
    /// Teleportation potion.
    ///
    /// Direction: Server <-> Client.
    pub struct TeleportationPotion {
        const TAG = 73;

        pub ty: TeleportationMethod,
    }
}
