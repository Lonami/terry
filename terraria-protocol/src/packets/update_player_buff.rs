use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update player buffs (and debuffs).
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UpdatePlayerBuff {
    pub player_id: u8,
    pub buffs: [u16; 22],
}

impl PacketBody for UpdatePlayerBuff {
    const TAG: u8 = 50;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        self.buffs.iter().for_each(|b| cursor.write(b));
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let player_id = cursor.read();
        let mut buffs = [0; 22];
        buffs.iter_mut().for_each(|b| *b = cursor.read());
        Self { player_id, buffs }
    }
}
