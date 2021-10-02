use crate::packets::packet_struct;

packet_struct! {
    /// Tweak Item (FKA. Alter Item Drop).
    ///
    /// Direction: Server -> Client.
    pub struct TweakItem {
        const TAG = 88;

        pub item_index: i16,
        /// BitFlags: 1 = Color, 2 = Damage, 4 = Knockback, 8 = UseAnimation, 16 = UseTime, 32 = Shoot, 64 = ShootSpeed, 128 = NextFlags
        pub flags1: u8,
        /// if Flags1.Color
        pub packed_color_value: u32,
        /// if Flags1.Damage
        pub damage: u16,
        /// if Flags1.Knockback
        pub knockback: f32,
        /// if Flags1.UseAnimation
        pub use_animation: u16,
        /// if Flags1.UseTime
        pub use_time: u16,
        /// if Flags1.Shoot
        pub shoot: i16,
        /// if Flags1.ShootSpeed
        pub shootspeed: f32,
        /// if Flags1.NextFlags, BitFlags: 1 = Width, 2 = Height, 4 = Scale, 8 = Ammo, 16 = UseAmmo, 32 = NotAmmo
        pub flags2: u8,
        /// if Flags2.Width
        pub width: i16,
        /// if Flags2.Height
        pub height: i16,
        /// if Flags2.Scale
        pub scale: f32,
        /// If Flags2.Ammo
        pub ammo: i16,
        /// If Flags2.UseAmmo
        pub useammo: i16,
        /// If Flags2.NotAmmo
        pub notammo: bool,
    }
}
