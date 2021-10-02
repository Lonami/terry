use crate::packets::packet_struct;
use crate::structures::{NetString, Vec2, RGB};

packet_struct! {
    /// Combat text string.
    ///
    /// Direction: Client <-> Server.
    pub struct CombatText {
        const TAG = 119;

        pub pos: Vec2,
        pub color: RGB,
        pub combat_text: NetString,
    }
}
