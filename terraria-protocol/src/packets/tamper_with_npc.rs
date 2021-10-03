use crate::serde::{PacketBody, Result, SliceCursor};

/// Tamper with a NPC.
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct TamperWithNpc {
    pub npc_id: u16,
    pub immunity_time: Option<i32>,
    /// Set to -1 for immunity from all players
    pub immunity_player_id: i16,
}

impl PacketBody for TamperWithNpc {
    const TAG: u8 = 131;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.npc_id)?;
        cursor.write(&self.immunity_time.is_some())?;
        if let Some(time) = self.immunity_time {
            cursor.write(&time)?;
        }
        cursor.write(&self.immunity_player_id)?;
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        Ok(Self {
            npc_id: cursor.read()?,
            immunity_time: cursor.read::<bool>()?.then(|| cursor.read()).transpose()?,
            immunity_player_id: cursor.read()?,
        })
    }
}
