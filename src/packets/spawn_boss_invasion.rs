use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Spawn Boss Invasion.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct SpawnBossInvasion {
    pub player_id: i16,
    /// Negative Values: -1 = GoblinInvasion, -2 = FrostInvasion, -3 = PirateInvasion, -4 = PumpkinMoon, -5 = SnowMoon, -6 = Eclipse, -7 = Martian Moon, -8 = Impending Doom, -10 = Blood Moon, -11 = Combat Book Used, -12 = Bought Cat, -13 = Bought Dog, -14 = Bought Bunny, Positive Values: Spawns any of these NPCs: 4, 13, 50, 126, 125, 134, 127, 128, 131, 129, 130, 222, 245, 266, 370, 657
    pub type: i16,
}

impl PacketBody for SpawnBossInvasion {
    const TAG: u8 = 61;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.type);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            type: cursor.read(),
        }
    }
}
