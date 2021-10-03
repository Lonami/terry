use crate::packets::packet_struct;
use crate::structures::serializable_enum;

serializable_enum! {
    pub enum EntityType: u8 {
        TrainingDummy = 0,
        ItemFrame = 1,
        LogicSensor = 2,
    }
}

packet_struct! {
    /// Place a tile entity.
    ///
    /// Direction: Client -> Server.
    pub struct PlaceTileEntity {
        const TAG = 87;

        pub x: i16,
        pub y: i16,
        pub tile_entity_type: EntityType,
    }
}
