use crate::serde::packet_struct;

packet_struct! {
    /// Wired Cannon Shot.
    ///
    /// Direction: Server -> Client.
    pub struct WiredCannonShot {
        const TAG = 108;

        pub damage: i16,
        pub knockback: f32,
        pub x: i16,
        pub y: i16,
        pub angle: i16,
        pub ammo: i16,
        /// Shooter's Player ID
        pub player_id: u8,
    }
}
