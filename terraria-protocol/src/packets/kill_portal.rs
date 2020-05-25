use crate::packets::PacketBody;
use crate::SliceCursor;

/// Kill a portal.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct KillPortal {
    pub projectile_owner: u16,
    pub projectile_ai: u8,
}

impl PacketBody for KillPortal {
    const TAG: u8 = 95;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.projectile_owner);
        cursor.write(&self.projectile_ai);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            projectile_owner: cursor.read(),
            projectile_ai: cursor.read(),
        }
    }
}
