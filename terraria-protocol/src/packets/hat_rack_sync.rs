use crate::packets::packet_struct;

packet_struct! {
    /// Hat rack item sync.
    ///
    /// Direction: Client <-> Server.
    pub struct HatRackSync {
        const TAG = 124;

        pub player_id: u8,
        pub tileentityid: i32,
        pub item_index: u8,
        pub item_id: u16,
        pub stack: u16,
        pub prefix: u8,
    }
}
