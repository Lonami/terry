use crate::packets::PacketBody;
use crate::SliceCursor;

/// Special NPC Effect.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SpecialNpcEffect {
    pub player_id: u8,
    /// Values: 1 = Spawn Skeletron, 2 = Cause sound at player, 3 = Start Sundialing (Only works if server is receiving), 4 = BigMimcSpawnSmoke
    pub ty: u8,
}

impl PacketBody for SpecialNpcEffect {
    const TAG: u8 = 51;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.ty);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            ty: cursor.read(),
        }
    }
}
