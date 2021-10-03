use crate::packets::PacketBody;
use crate::SliceCursor;

/// Sync the type of a cavern monster.
///
/// Direction: Client <-> Server.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SyncMonsterType {
    pub net_id: [[u16; 3]; 2],
}

impl PacketBody for SyncMonsterType {
    const TAG: u8 = 136;

    fn write_body(&self, cursor: &mut SliceCursor) {
        self.net_id
            .iter()
            .for_each(|row| row.iter().for_each(|x| cursor.write(x)));
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            net_id: [
                [cursor.read(), cursor.read(), cursor.read()],
                [cursor.read(), cursor.read(), cursor.read()],
            ],
        }
    }
}
