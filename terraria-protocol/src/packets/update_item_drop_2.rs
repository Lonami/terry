use crate::serde::packet_struct;
use crate::structures::Vec2;

packet_struct! {
    /// Update Item Drop 2.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct UpdateItemDrop2 {
        const TAG = 90;

        /// Should set to null ``if item_id < 400 && item_net_id == 0`.
        /// Otherwise, ``if item_id = 400`` it's a new item.
        pub item_id: i16,
        pub pos: Vec2,
        pub vel: Vec2,
        pub stack_size: i16,
        pub prefix: u8,
        /// If 0, set ``own_ignore = 0`` and ``own_time = 100``
        pub no_delay: u8,
        pub item_net_id: i16,
    }
}
