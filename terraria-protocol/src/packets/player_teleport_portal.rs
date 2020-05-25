use crate::packets::PacketBody;
use crate::SliceCursor;
use crate::structures::Vec2;

/// Teleport a player through a portal.
///
/// Direction: Server <-> Client.
#[derive(Debug)]
pub struct PlayerTeleportPortal {
    pub player_id: u8,
    pub portal_color_index: i16,
    pub pos: Vec2,
    pub vel: Vec2,
}

impl PacketBody for PlayerTeleportPortal {
    const TAG: u8 = 96;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.portal_color_index);
        cursor.write(&self.pos);
        cursor.write(&self.vel);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            portal_color_index: cursor.read(),
            pos: cursor.read(),
            vel: cursor.read(),
        }
    }
}
