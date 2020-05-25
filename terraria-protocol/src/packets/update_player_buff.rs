use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update Player Buff.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdatePlayerBuff {
    pub player_id: u8,
    pub bufftype: [u16; 22],
}

impl PacketBody for UpdatePlayerBuff {
    const TAG: u8 = 50;

    fn write_body(&self, _cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        todo!()
    }
}
