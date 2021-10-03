use crate::serde::{PacketBody, Result, SliceCursor};

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

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.player_id)?;
        for b in self.buffs.iter() {
            cursor.write(b)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let player_id = cursor.read()?;
        let mut buffs = [0; 22];
        for b in buffs.iter_mut() {
            *b = cursor.read()?;
        }
        Ok(Self { player_id, buffs })
    }
}
