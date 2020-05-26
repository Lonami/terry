use crate::packets::PacketBody;
use crate::structures::Vec2;
use crate::SliceCursor;

/// Projectile update.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct ProjectileUpdate {
    pub projectile_id: i16,
    pub pos: Vec2,
    pub vel: Vec2,
    /// Player ID.
    pub owner: u8,
    pub ty: i16,
    /// Can have up to 2 values.
    pub ai: Vec<f32>,
    pub damage: Option<i16>,
    pub knockback: Option<f32>,
    pub original_damage: Option<i16>,
    pub proj_uuid: Option<i16>,
}

impl PacketBody for ProjectileUpdate {
    const TAG: u8 = 27;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.projectile_id);
        cursor.write(&self.pos);
        cursor.write(&self.vel);
        cursor.write(&self.owner);
        cursor.write(&self.ty);
        cursor.write(
            &(0 // projectile flags
            | if self.ai.len() >= 1 { 0x01 } else { 0 }
            | if self.ai.len() >= 2 { 0x02 } else { 0 }
            | if self.damage.is_some() { 0x10 } else { 0 }
            | if self.knockback.is_some() { 0x20 } else { 0 }
            | if self.original_damage.is_some() { 0x40 } else { 0 }
            | if self.proj_uuid.is_some() { 0x80 } else { 0 }),
        );
        self.ai.iter().take(2).for_each(|a| cursor.write(a));
        if let Some(damage) = self.damage {
            cursor.write(&damage);
        }
        if let Some(knockback) = self.knockback {
            cursor.write(&knockback);
        }
        if let Some(original_damage) = self.original_damage {
            cursor.write(&original_damage);
        }
        if let Some(proj_uuid) = self.proj_uuid {
            cursor.write(&proj_uuid);
        }
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let projectile_id = cursor.read();
        let pos = cursor.read();
        let vel = cursor.read();
        let owner = cursor.read();
        let ty = cursor.read();
        let proj_flags = cursor.read::<u8>();
        let mut ai = Vec::with_capacity(2);
        if proj_flags & 0x01 != 0 {
            ai.push(cursor.read());
        }
        if proj_flags & 0x02 != 0 {
            ai.push(cursor.read());
        }
        let damage = if proj_flags & 0x10 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let knockback = if proj_flags & 0x20 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let original_damage = if proj_flags & 0x40 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let proj_uuid = if proj_flags & 0x80 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        Self {
            projectile_id,
            pos,
            vel,
            owner,
            ty,
            ai,
            damage,
            knockback,
            original_damage,
            proj_uuid,
        }
    }
}
