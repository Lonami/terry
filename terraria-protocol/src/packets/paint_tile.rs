use crate::serde::packet_struct;

packet_struct! {
    /// Paint a tile.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PaintTile {
        const TAG = 63;

        pub x: i16,
        pub y: i16,
        pub color: u8,
    }
}
