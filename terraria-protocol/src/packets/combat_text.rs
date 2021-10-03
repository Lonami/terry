use crate::serde::packet_struct;
use crate::structures::{NetString, Rgb, Vec2};

packet_struct! {
    /// Combat text string.
    ///
    /// Direction: Client <-> Server.
    pub struct CombatText {
        const TAG = 119;

        pub pos: Vec2,
        pub color: Rgb,
        pub combat_text: NetString,
    }
}
