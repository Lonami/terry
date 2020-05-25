use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// MinionAttackTargetUpdate.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct MinionAttackTargetUpdate {
    pub player_id: u8,
    pub minion_attack_target: i16,
}

impl PacketBody for MinionAttackTargetUpdate {
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
