use crate::serde::{packet_struct, serializable_enum};

serializable_enum! {
    pub enum ChestAction: u8 {
        PlaceChest = 0,
        KillChest = 1,
        PlaceDresser = 2,
        KillDresser = 3,
        PlaceContainers = 4,
        KillContainers = 5,
    }
}

packet_struct! {
    /// Place a chest.
    ///
    /// Direction: Server <-> Client.
    pub struct PlaceChest {
        const TAG = 34;

        pub action: ChestAction,
        pub tile_x: i16,
        pub tile_y: i16,
        /// Frame X (chest type)
        pub style: i16,
        /// ID if client is receiving packet, else 0
        pub chest_id_to_destroy: i16,
    }
}
