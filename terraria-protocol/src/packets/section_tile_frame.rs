use crate::packets::packet_struct;

packet_struct! {
    /// Section tile frame.
    ///
    /// Direction: Server -> Client.
    pub struct SectionTileFrame {
        const TAG = 11;

        pub start_x: i16,
        pub start_y: i16,
        pub end_x: i16,
        pub end_y: i16,
    }
}
