use crate::packets::PacketBody;
use crate::structures::Vec2;
use crate::SliceCursor;

/// No description known yet.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerNpcTeleport {
    /// BitFlags: 0 = Player Teleport (Neither 1 or 2), 1 = NPC Teleport, 2 = Player Teleport to Other Player, 4 = GetPositionFromTarget, 8 = HasExtraInfo
    pub flags: u8,
    pub target_id: i16,
    pub pos: Vec2,
    pub style: u8,
    /// Only sent if HasExtraInfo flag is true
    pub extrainfo: i32,
}

impl PacketBody for PlayerNpcTeleport {
    const TAG: u8 = 65;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.flags);
        cursor.write(&self.target_id);
        cursor.write(&self.pos);
        cursor.write(&self.style);
        cursor.write(&self.extrainfo);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            flags: cursor.read(),
            target_id: cursor.read(),
            pos: cursor.read(),
            style: cursor.read(),
            extrainfo: cursor.read(),
        }
    }
}
