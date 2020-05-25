use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Set player stealth.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SetPlayerStealth {
    pub player: u8,
    pub stealth: i32, /* single */
}

impl PacketBody for SetPlayerStealth {
    const TAG: u8 = 84;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player);
        cursor.write(&self.stealth);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player: cursor.read(),
            stealth: cursor.read(),
        }
    }
}
