use crate::packets::packet_struct;
use crate::structures::{Deserializable, Serializable, SliceCursor};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Invader {
    GoblinInvasion,
    FrostInvasion,
    PirateInvasion,
    PumpkinMoon,
    SnowMoon,
    Eclipse,
    MartianMoon,
    ImpendingDoom,
    BloodMoon,
    CombatBookUsed,
    BoughtCat,
    BoughtDog,
    BoughtBunny,
    /// One of: 4, 13, 50, 126, 125, 134, 127, 128, 131, 129, 130, 222, 245, 266, 370, 657
    SpawnNpc(i16),
}

impl Default for Invader {
    fn default() -> Self {
        Self::GoblinInvasion
    }
}

impl Serializable for Invader {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&match self {
            Self::GoblinInvasion => -1,
            Self::FrostInvasion => -2,
            Self::PirateInvasion => -3,
            Self::PumpkinMoon => -4,
            Self::SnowMoon => -5,
            Self::Eclipse => -6,
            Self::MartianMoon => -7,
            Self::ImpendingDoom => -8,
            Self::BloodMoon => -10,
            Self::CombatBookUsed => -11,
            Self::BoughtCat => -12,
            Self::BoughtDog => -13,
            Self::BoughtBunny => -14,
            Self::SpawnNpc(npc) => *npc,
        });
    }
}

impl Deserializable for Invader {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        match cursor.read::<i16>() {
            -1 => Self::GoblinInvasion,
            -2 => Self::FrostInvasion,
            -3 => Self::PirateInvasion,
            -4 => Self::PumpkinMoon,
            -5 => Self::SnowMoon,
            -6 => Self::Eclipse,
            -7 => Self::MartianMoon,
            -8 => Self::ImpendingDoom,
            -10 => Self::BloodMoon,
            -11 => Self::CombatBookUsed,
            -12 => Self::BoughtCat,
            -13 => Self::BoughtDog,
            -14 => Self::BoughtBunny,
            npc => Self::SpawnNpc(npc),
        }
    }
}

packet_struct! {
    /// Spawn a boss invasion.
    ///
    /// Direction: Client -> Server.
    pub struct SpawnBossInvasion {
        const TAG = 61;

        pub player_id: i16,
        pub ty: Invader,
    }
}
