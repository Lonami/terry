use crate::packets::packet_struct;

packet_struct! {
    /// Create a temporary animation.
    ///
    /// Direction: Server -> Client.
    pub struct CreateTemporaryAnimation {
        const TAG = 77;

        pub animation_type: i16,
        pub tile_type: u16,
        pub x: i16,
        pub y: i16,
    }
}
