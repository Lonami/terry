use crate::serde::{PacketBody, Result, SliceCursor};

/// Update NPC Buff.
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UpdateNpcBuff {
    pub npc_id: i16,
    /// Tuples of ``(buff id, time)``
    pub buff_times: [(u16, i16); 5],
}

impl PacketBody for UpdateNpcBuff {
    const TAG: u8 = 54;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.npc_id)?;
        for (b, t) in self.buff_times.iter() {
            cursor.write(b)?;
            cursor.write(t)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let npc_id = cursor.read()?;
        let mut buff_times = [(0, 0); 5];
        for tuple in buff_times.iter_mut() {
            *tuple = (cursor.read()?, cursor.read()?);
        }
        Ok(Self { npc_id, buff_times })
    }
}
