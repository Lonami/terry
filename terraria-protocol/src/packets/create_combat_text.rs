use crate::packets::packet_struct;
use crate::structures::{Vec2, RGB};

packet_struct! {
    /// Create combat text.
    ///
    /// Direction: Server -> Client.
    pub struct CreateCombatText {
        const TAG = 81;

        pub pos: Vec2,
        pub color: RGB,
        pub heal_amount: i32,
    }
}
