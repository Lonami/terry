use crate::packets::PacketBody;
use crate::serialization::{Serializable, Deserializable, SliceCursor};

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum SpawnContext {
    ReviveFromDeath = 0,
    SpawningIntoWorld = 1,
    RecallFromItem = 2,
}

impl Serializable for SpawnContext {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
    }
}

impl Deserializable for SpawnContext {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        match cursor.read::<u8>() {
            0 => SpawnContext::ReviveFromDeath,
            1 => SpawnContext::SpawningIntoWorld,
            2 => SpawnContext::RecallFromItem,
            n => panic!(format!("invalid respawn contet {}", n)),
        }
    }
}

/// Spawn a player.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct SpawnPlayer {
    pub player_id: u8,
    pub spawn_x: i16,
    pub spawn_y: i16,
    /// If > 0, then player is still dead
    pub respawn_time_remaining: i32,
    /// Enum: 0 = ReviveFromDeath, 1 = SpawningIntoWorld, 2 = RecallFromItem
    pub player_spawn_context: u8,
}

impl PacketBody for SpawnPlayer {
    const TAG: u8 = 12;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.spawn_x);
        cursor.write(&self.spawn_y);
        cursor.write(&self.respawn_time_remaining);
        cursor.write(&self.player_spawn_context);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            spawn_x: cursor.read(),
            spawn_y: cursor.read(),
            respawn_time_remaining: cursor.read(),
            player_spawn_context: cursor.read(),
        }
    }
}
