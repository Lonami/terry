use crate::packets::PacketBody;
use crate::structures::PlayerDeathReason;
use crate::SliceCursor;

/// Player death (version 2).
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct PlayerDeath {
    pub player_id: u8,
    pub player_death_reason: PlayerDeathReason,
    pub damage: i16,
    pub hit_direction: u8,
    /// BitFlags: 1 = PvP
    pub flags: u8,
}

impl PacketBody for PlayerDeath {
    const TAG: u8 = 118;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.player_death_reason);
        cursor.write(&self.damage);
        cursor.write(&self.hit_direction);
        cursor.write(&self.flags);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            player_death_reason: cursor.read(),
            damage: cursor.read(),
            hit_direction: cursor.read(),
            flags: cursor.read(),
        }
    }
}
