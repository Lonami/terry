use crate::packets::packet_struct;

packet_struct! {
    /// Growth sound effects.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct GrowFx {
        const TAG = 112;

        /// 1 = Tree Growth Effects, 2 = Fairy Effects
        pub effectflags: u8,
        pub x: i32,
        pub y: i32,
        pub height: u8,
        pub tree_gore: i16,
    }
}
