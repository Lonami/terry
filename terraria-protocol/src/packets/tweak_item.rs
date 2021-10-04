use crate::serde::{fixup_flags, serializable_bitflags, PacketBody, Result, SliceCursor};

serializable_bitflags! {
    pub struct TweakFlags: u8 {
        const COLOR = 0x01;
        const DAMAGE = 0x02;
        const KNOCKBACK = 0x04;
        const USE_ANIMATON = 0x08;
        const USE_TIME = 0x10;
        const SHOOT = 0x20;
        const SHOOT_SPEED = 0x40;
        const NEXT_FLAGS = 0x80;
    }
}

serializable_bitflags! {
    pub struct TweakExtraFlags: u8 {
        const WIDTH = 0x01;
        const HEIGHT = 0x02;
        const SCALE = 0x04;
        const AMMO = 0x08;
        const USE_AMMO = 0x10;
        const NOT_AMMO = 0x20;
    }
}

/// Tweak Item (formerly known as Alter Item Drop).
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct TweakItem {
    pub item_index: u16,
    pub flags: TweakFlags,
    pub packed_color_value: Option<u32>,
    pub damage: Option<u16>,
    pub knockback: Option<f32>,
    pub use_animation: Option<u16>,
    pub use_time: Option<u16>,
    pub shoot: Option<i16>,
    pub shootspeed: Option<f32>,
    pub extra_flags: TweakExtraFlags,
    pub width: Option<i16>,
    pub height: Option<i16>,
    pub scale: Option<f32>,
    pub ammo: Option<i16>,
    pub use_ammo: Option<i16>,
    pub not_ammo: Option<bool>,
}

impl Eq for TweakItem {}

impl PacketBody for TweakItem {
    const TAG: u8 = 88;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        let fixed_extra = fixup_flags!(TweakExtraFlags where {
            self.width.is_some() => WIDTH,
            self.height.is_some() => HEIGHT,
            self.scale.is_some() => SCALE,
            self.ammo.is_some() => AMMO,
            self.use_ammo.is_some() => USE_AMMO,
            self.not_ammo.is_some() => NOT_AMMO,
        } in self.extra_flags);

        cursor.write(&self.item_index)?;
        cursor.write(&fixup_flags!(TweakFlags where {
            self.packed_color_value.is_some() => COLOR,
            self.damage.is_some() => DAMAGE,
            self.knockback.is_some() => KNOCKBACK,
            self.use_animation.is_some() => USE_ANIMATON,
            self.use_time.is_some() => USE_TIME,
            self.shoot.is_some() => SHOOT,
            self.shootspeed.is_some() => SHOOT_SPEED,
            !fixed_extra.is_empty() => NEXT_FLAGS,
        } in self.flags))?;
        if let Some(packed_color_value) = self.packed_color_value {
            cursor.write(&packed_color_value)?;
        }
        if let Some(damage) = self.damage {
            cursor.write(&damage)?;
        }
        if let Some(knockback) = self.knockback {
            cursor.write(&knockback)?;
        }
        if let Some(use_animation) = self.use_animation {
            cursor.write(&use_animation)?;
        }
        if let Some(use_time) = self.use_time {
            cursor.write(&use_time)?;
        }
        if let Some(shoot) = self.shoot {
            cursor.write(&shoot)?;
        }
        if let Some(shootspeed) = self.shootspeed {
            cursor.write(&shootspeed)?;
        }
        if !fixed_extra.is_empty() {
            cursor.write(&fixed_extra)?;
        }
        if let Some(width) = self.width {
            cursor.write(&width)?;
        }
        if let Some(height) = self.height {
            cursor.write(&height)?;
        }
        if let Some(scale) = self.scale {
            cursor.write(&scale)?;
        }
        if let Some(ammo) = self.ammo {
            cursor.write(&ammo)?;
        }
        if let Some(use_ammo) = self.use_ammo {
            cursor.write(&use_ammo)?;
        }
        if let Some(not_ammo) = self.not_ammo {
            cursor.write(&not_ammo)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let item_index = cursor.read()?;

        let flags = cursor.read::<TweakFlags>()?;
        let packed_color_value = flags
            .contains(TweakFlags::COLOR)
            .then(|| cursor.read())
            .transpose()?;
        let damage = flags
            .contains(TweakFlags::DAMAGE)
            .then(|| cursor.read())
            .transpose()?;
        let knockback = flags
            .contains(TweakFlags::KNOCKBACK)
            .then(|| cursor.read())
            .transpose()?;
        let use_animation = flags
            .contains(TweakFlags::USE_ANIMATON)
            .then(|| cursor.read())
            .transpose()?;
        let use_time = flags
            .contains(TweakFlags::USE_TIME)
            .then(|| cursor.read())
            .transpose()?;
        let shoot = flags
            .contains(TweakFlags::SHOOT)
            .then(|| cursor.read())
            .transpose()?;
        let shootspeed = flags
            .contains(TweakFlags::SHOOT_SPEED)
            .then(|| cursor.read())
            .transpose()?;

        let extra_flags = flags
            .contains(TweakFlags::NEXT_FLAGS)
            .then(|| cursor.read::<TweakExtraFlags>())
            .transpose()?
            .unwrap_or_else(TweakExtraFlags::empty);
        let width = extra_flags
            .contains(TweakExtraFlags::WIDTH)
            .then(|| cursor.read())
            .transpose()?;
        let height = extra_flags
            .contains(TweakExtraFlags::HEIGHT)
            .then(|| cursor.read())
            .transpose()?;
        let scale = extra_flags
            .contains(TweakExtraFlags::SCALE)
            .then(|| cursor.read())
            .transpose()?;
        let ammo = extra_flags
            .contains(TweakExtraFlags::AMMO)
            .then(|| cursor.read())
            .transpose()?;
        let use_ammo = extra_flags
            .contains(TweakExtraFlags::USE_AMMO)
            .then(|| cursor.read())
            .transpose()?;
        let not_ammo = extra_flags
            .contains(TweakExtraFlags::NOT_AMMO)
            .then(|| cursor.read())
            .transpose()?;

        Ok(Self {
            item_index,
            flags,
            packed_color_value,
            damage,
            knockback,
            use_animation,
            use_time,
            shoot,
            shootspeed,
            extra_flags,
            width,
            height,
            scale,
            ammo,
            use_ammo,
            not_ammo,
        })
    }
}
