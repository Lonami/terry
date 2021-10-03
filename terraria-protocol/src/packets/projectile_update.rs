use crate::serde::{serializable_bitflags, PacketBody, Result, SliceCursor};
use crate::structures::Vec2;

serializable_bitflags! {
    pub struct ProjectileFlag: u8 {
        const AI1 = 0x01;
        const AI2 = 0x02;
        const HAS_DAMAGE = 0x10;
        const HAS_KNOCKBACK = 0x20;
        const HAS_ORIG_DAMAGE = 0x40;
        const HAS_UUID = 0x80;
    }
}

/// Projectile update.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug, PartialEq, Default, Clone)]
pub struct ProjectileUpdate {
    pub projectile_id: i16,
    pub pos: Vec2,
    pub vel: Vec2,
    /// Player ID
    pub owner: u8,
    pub ty: i16,
    pub flags: ProjectileFlag,
    /// Only has meaningful values if the corresponding flag is set
    pub ai: [f32; 2],
    pub damage: Option<i16>,
    pub knockback: Option<f32>,
    pub original_damage: Option<i16>,
    pub proj_uuid: Option<i16>,
}

impl Eq for ProjectileUpdate {}

impl PacketBody for ProjectileUpdate {
    const TAG: u8 = 27;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.projectile_id)?;
        cursor.write(&self.pos)?;
        cursor.write(&self.vel)?;
        cursor.write(&self.owner)?;
        cursor.write(&self.ty)?;
        cursor.write(&self.flags)?;
        if self.flags.contains(ProjectileFlag::AI1) {
            cursor.write(&self.ai[0])?;
        }
        if self.flags.contains(ProjectileFlag::AI2) {
            cursor.write(&self.ai[1])?;
        }
        if let Some(damage) = self.damage {
            cursor.write(&damage)?;
        }
        if let Some(knockback) = self.knockback {
            cursor.write(&knockback)?;
        }
        if let Some(original_damage) = self.original_damage {
            cursor.write(&original_damage)?;
        }
        if let Some(proj_uuid) = self.proj_uuid {
            cursor.write(&proj_uuid)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let projectile_id = cursor.read()?;
        let pos = cursor.read()?;
        let vel = cursor.read()?;
        let owner = cursor.read()?;
        let ty = cursor.read()?;
        let flags = cursor.read::<ProjectileFlag>()?;

        let mut ai = [0f32; 2];
        if flags.contains(ProjectileFlag::AI1) {
            ai[0] = cursor.read()?;
        }
        if flags.contains(ProjectileFlag::AI2) {
            ai[1] = cursor.read()?;
        }

        let damage = flags
            .contains(ProjectileFlag::HAS_DAMAGE)
            .then(|| cursor.read())
            .transpose()?;

        let knockback = flags
            .contains(ProjectileFlag::HAS_KNOCKBACK)
            .then(|| cursor.read())
            .transpose()?;

        let original_damage = flags
            .contains(ProjectileFlag::HAS_ORIG_DAMAGE)
            .then(|| cursor.read())
            .transpose()?;

        let proj_uuid = flags
            .contains(ProjectileFlag::HAS_UUID)
            .then(|| cursor.read())
            .transpose()?;

        Ok(Self {
            projectile_id,
            pos,
            vel,
            owner,
            ty,
            flags,
            ai,
            damage,
            knockback,
            original_damage,
            proj_uuid,
        })
    }
}
