use crate::serde::packet_struct;

packet_struct! {
    /// Paint a wall.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PaintWall {
        const TAG = 64;

        pub x: i16,
        pub y: i16,
        pub color: u8,
    }
}
