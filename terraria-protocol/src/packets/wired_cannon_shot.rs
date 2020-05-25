use crate::packets::PacketBody;
use crate::SliceCursor;

/// Wired Cannon Shot.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct WiredCannonShot {
    pub damage: i16,
    pub knockback: f32,
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub ammo: i16,
    /// Shooter's Player ID
    pub player_id: u8,
}

impl PacketBody for WiredCannonShot {
    const TAG: u8 = 108;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.damage);
        cursor.write(&self.knockback);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.angle);
        cursor.write(&self.ammo);
        cursor.write(&self.player_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            damage: cursor.read(),
            knockback: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            angle: cursor.read(),
            ammo: cursor.read(),
            player_id: cursor.read(),
        }
    }
}
