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
            0 => NetStringMode::ReviveFromDeath,
            1 => NetStringMode::SpawningIntoWorld,
            2 => NetStringMode::RecallFromItem,
            n => panic!(format!("invalid respawn contet {}", n)),
        }
    }
}

/// Puts a player on spawn, either by login, death or recall.
///
/// Direction: Client to Server.
#[derive(Debug)]
pub struct SpawnPlayer {
    pub player: u8,
    pub x: i16,
    pub y: i16,
    /// Greater than 0 indicates the player is still dead.
    pub respawn_timer: u32,
    pub context: SpawnContext,
}

impl PacketBody for SpawnPlayer {
    const TAG: u8 = 12;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.respawn_timer);
        cursor.write(&self.context);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            id: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            respawn_timer: cursor.read(),
            context: cursor.read(),
        }
    }
}
