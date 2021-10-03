use crate::serde::packet_struct;
use crate::structures::{Rgb, Vec2};

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
