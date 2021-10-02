use crate::packets::packet_struct;

packet_struct! {
    /// Place a tile entity.
    ///
    /// Direction: Client -> Server.
    pub struct PlaceTileEntity {
        const TAG = 87;

        pub x: i16,
        pub y: i16,
        /// 2 = Logic Sensor 1 = Item Frame 0 = Training Dummy
        pub tileentitytype: u8,
    }
}
