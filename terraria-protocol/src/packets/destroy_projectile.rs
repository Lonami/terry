use crate::packets::PacketBody;
use crate::SliceCursor;

/// Destroy a projectile.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct DestroyProjectile {
    pub projectile_id: i16,
    /// Player ID.
    pub owner: u8,
}

impl PacketBody for DestroyProjectile {
    const TAG: u8 = 29;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.projectile_id);
        cursor.write(&self.owner);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            projectile_id: cursor.read(),
            owner: cursor.read(),
        }
    }
}
