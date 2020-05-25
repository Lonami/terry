use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Projectile update.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct ProjectileUpdate {
    pub projectile_id: i16,
    pub position_x: i32, /* single */
    pub position_y: i32, /* single */
    pub velocity_x: i32, /* single */
    pub velocity_y: i32, /* single */
    /// Player ID
    pub owner: u8,
    pub ty: i16,
    /// BitFlags: 1 = AI
    pub projflags: u8,
    /// Only sent if AI
    pub ai0: i32, /* single */
    /// Only sent if AI
    pub ai1: i32, /* single */
    /// Only sent if Damage flag is true
    pub damage: i16,
    /// Only sent if Knockback flag is true
    pub knockback: i32, /* single */
    /// Only sent if OriginalDamage flag is true
    pub original_damage: i16,
    /// Only sent if ProjUUID flag is true
    pub proj_uuid: i16,
}

impl PacketBody for ProjectileUpdate {
    const TAG: u8 = 27;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.projectile_id);
        cursor.write(&self.position_x);
        cursor.write(&self.position_y);
        cursor.write(&self.velocity_x);
        cursor.write(&self.velocity_y);
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
            position_x: cursor.read(),
            position_y: cursor.read(),
            velocity_x: cursor.read(),
            velocity_y: cursor.read(),
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
