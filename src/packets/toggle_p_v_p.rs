use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Toggle P V P.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct TogglePVP {
    pub player_id: u8,
    pub pvp_enabled: bool,
}

impl PacketBody for TogglePVP {
    const TAG: u8 = 30;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.pvp_enabled);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            pvp_enabled: cursor.read(),
        }
    }
}
