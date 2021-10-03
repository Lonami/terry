use crate::serde::{PacketBody, Result, SliceCursor};

/// Sync the type of a cavern monster.
///
/// Direction: Client <-> Server.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SyncMonsterType {
    pub net_id: [[u16; 3]; 2],
}

impl PacketBody for SyncMonsterType {
    const TAG: u8 = 136;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        for row in self.net_id.iter() {
            for x in row.iter() {
                cursor.write(x)?;
            }
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        Ok(Self {
            net_id: [
                [cursor.read()?, cursor.read()?, cursor.read()?],
                [cursor.read()?, cursor.read()?, cursor.read()?],
            ],
        })
    }
}
