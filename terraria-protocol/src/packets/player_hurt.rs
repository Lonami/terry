use crate::packets::PacketBody;
use crate::structures::PlayerDeathReason;
use crate::SliceCursor;

/// Player hurt (version 2).
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct PlayerHurt {
    pub player_id: u8,
    pub player_death_reason: PlayerDeathReason,
    pub damage: i16,
    pub hit_direction: u8,
    /// BitFlags: 1 = Crit, 2 = PvP
    pub flags: u8,
    pub cooldown_counter: i8,
}

impl PacketBody for PlayerHurt {
    const TAG: u8 = 117;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.player_death_reason);
        cursor.write(&self.damage);
        cursor.write(&self.hit_direction);
        cursor.write(&self.flags);
        cursor.write(&self.cooldown_counter);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            player_death_reason: cursor.read(),
            damage: cursor.read(),
            hit_direction: cursor.read(),
            flags: cursor.read(),
            cooldown_counter: cursor.read(),
        }
    }
}
