use crate::packets::PacketBody;
use crate::SliceCursor;
use crate::structures::Vec2;

/// Projectile update.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct ProjectileUpdate {
    pub projectile_id: i16,
    pub pos: Vec2,
    pub vel: Vec2,
    /// Player ID
    pub owner: u8,
    pub ty: i16,
    /// BitFlags: 1 = AI
    pub projflags: u8,
    /// Only sent if AI
    pub ai0: f32,
    /// Only sent if AI
    pub ai1: f32,
    /// Only sent if Damage flag is true
    pub damage: i16,
    /// Only sent if Knockback flag is true
    pub knockback: f32,
    /// Only sent if OriginalDamage flag is true
    pub original_damage: i16,
    /// Only sent if ProjUUID flag is true
    pub proj_uuid: i16,
}

impl PacketBody for ProjectileUpdate {
    const TAG: u8 = 27;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.projectile_id);
        cursor.write(&self.pos);
        cursor.write(&self.vel);
        cursor.write(&self.owner);
        cursor.write(&self.ty);
        cursor.write(&self.projflags);
        cursor.write(&self.ai0);
        cursor.write(&self.ai1);
        cursor.write(&self.damage);
        cursor.write(&self.knockback);
        cursor.write(&self.original_damage);
        cursor.write(&self.proj_uuid);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            projectile_id: cursor.read(),
            pos: cursor.read(),
            vel: cursor.read(),
            owner: cursor.read(),
            ty: cursor.read(),
            projflags: cursor.read(),
            ai0: cursor.read(),
            ai1: cursor.read(),
            damage: cursor.read(),
            knockback: cursor.read(),
            original_damage: cursor.read(),
            proj_uuid: cursor.read(),
        }
    }
}
