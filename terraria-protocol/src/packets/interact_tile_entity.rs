use crate::serde::packet_struct;

packet_struct! {
    /// Request interaction with a tile entity.
    ///
    /// Direction: Client <-> Server.
    pub struct InteractTileEntity {
        const TAG = 122;

        pub tile_entity_id: i32,
        pub player_id: u8,
    }
}
