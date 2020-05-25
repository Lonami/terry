use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Tweak Item (FKA. Alter Item Drop).
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct TweakItem(FKA.AlterItemDrop) {
    pub item_index: i16,
    /// BitFlags: 1 = Color, 2 = Damage, 4 = Knockback, 8 = UseAnimation, 16 = UseTime, 32 = Shoot, 64 = ShootSpeed, 128 = NextFlags
    pub flags1: u8,
    /// if Flags1.Color
    pub packed_color_value: u32,
    /// if Flags1.Damage
    pub damage: u16,
    /// if Flags1.Knockback
    pub knockback: i32 /* single */ ,
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

impl PacketBody for TweakItem(FKA.AlterItemDrop) {
    const TAG: u8 = 88;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.item_index);
        cursor.write(&self.flags1);
        cursor.write(&self.packed_color_value);
        cursor.write(&self.damage);
        cursor.write(&self.knockback);
        cursor.write(&self.use_animation);
        cursor.write(&self.use_time);
        cursor.write(&self.shoot);
        cursor.write(&self.shootspeed);
        cursor.write(&self.flags2);
        cursor.write(&self.width);
        cursor.write(&self.height);
        cursor.write(&self.scale);
        cursor.write(&self.ammo);
        cursor.write(&self.useammo);
        cursor.write(&self.notammo);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            item_index: cursor.read(),
            flags1: cursor.read(),
            packed_color_value: cursor.read(),
            damage: cursor.read(),
            knockback: cursor.read(),
            use_animation: cursor.read(),
            use_time: cursor.read(),
            shoot: cursor.read(),
            shootspeed: cursor.read(),
            flags2: cursor.read(),
            width: cursor.read(),
            height: cursor.read(),
            scale: cursor.read(),
            ammo: cursor.read(),
            useammo: cursor.read(),
            notammo: cursor.read(),
        }
    }
}
