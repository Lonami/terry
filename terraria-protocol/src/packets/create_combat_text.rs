use crate::serde::packet_struct;
use crate::structures::{Vec2, Rgb};

packet_struct! {
    /// Create combat text.
    ///
    /// Direction: Server -> Client.
    pub struct CreateCombatText {
        const TAG = 81;

        pub pos: Vec2,
        pub color: Rgb,
        pub heal_amount: i32,
    }
}
