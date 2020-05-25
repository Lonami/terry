use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Set the player's team.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerTeam {
    pub player_id: u8,
    pub team: u8,
}

impl PacketBody for PlayerTeam {
    const TAG: u8 = 45;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.team);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            team: cursor.read(),
        }
    }
}
