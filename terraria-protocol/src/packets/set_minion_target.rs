use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Update a minion's attack target.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct SetMinionTarget {
    pub player_id: u8,
    pub minion_attack_target: i16,
}

impl PacketBody for SetMinionTarget {
    const TAG: u8 = 115;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.minion_attack_target);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            minion_attack_target: cursor.read(),
        }
    }
}
