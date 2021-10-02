use crate::{Deserializable, Serializable, SliceCursor};

use bitflags::bitflags;

bitflags! {
    pub struct DeathReason: u8 {
        const HAS_KILLER = 1;
        const HAS_KILLING = 2;
        const HAS_PROJECTILE_IDX = 4;
        const HAS_TYPE_OF_DEATH = 8;
        const HAS_PROJECTILE_TY = 16;
        const HAS_ITEM_TY = 32;
        const HAS_ITEM_PREFIX = 64;
        const HAS_CUSTOM_MSG = 128;
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum DeathType {
    FallDamage = 0,
    Drowning = 1,
    LavaDamage = 2,
    FallDamage2 = 3,
    DemonAltar = 4,
    CompanionCube = 6,
    Suffocation = 7,
    Burning = 8,
    PoisonVenom = 9,
    Electrified = 10,
    WofEscape = 11,
    WofLicked = 12,
    ChaosState = 13,
    ChaosStateMale = 14,
    ChaosStateFemale = 15,
}

impl Serializable for DeathType {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
    }
}

impl Deserializable for DeathType {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        match cursor.read::<u8>() {
            0 => DeathType::FallDamage,
            1 => DeathType::Drowning,
            2 => DeathType::LavaDamage,
            3 => DeathType::FallDamage,
            4 => DeathType::DemonAltar,
            6 => DeathType::CompanionCube,
            7 => DeathType::Suffocation,
            8 => DeathType::Burning,
            9 => DeathType::PoisonVenom,
            10 => DeathType::Electrified,
            11 => DeathType::WofEscape,
            12 => DeathType::WofLicked,
            13 => DeathType::ChaosState,
            14 => DeathType::ChaosStateMale,
            15 => DeathType::ChaosStateFemale,
            n => panic!("invalid death type {}", n),
        }
    }
}

#[derive(Debug)]
pub struct PlayerDeathReason {
    killer_player_id: Option<i16>,
    killing_npc_index: Option<i16>,
    projectile_index: Option<i16>,
    type_of_death: Option<DeathType>,
    projectile_type: Option<i16>,
    item_type: Option<i16>,
    item_prefix: Option<u8>,
    death_reason: Option<String>,
}

impl Serializable for PlayerDeathReason {
    fn serialize(&self, _cursor: &mut SliceCursor) {
        todo!()
    }
}

impl Deserializable for PlayerDeathReason {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        let reason = DeathReason::from_bits_truncate(cursor.read());

        let killer_player_id = reason
            .contains(DeathReason::HAS_KILLER)
            .then(|| cursor.read());

        let killing_npc_index = reason
            .contains(DeathReason::HAS_KILLING)
            .then(|| cursor.read());

        let projectile_index = reason
            .contains(DeathReason::HAS_PROJECTILE_IDX)
            .then(|| cursor.read());

        let type_of_death = reason
            .contains(DeathReason::HAS_TYPE_OF_DEATH)
            .then(|| cursor.read());

        let projectile_type = reason
            .contains(DeathReason::HAS_PROJECTILE_TY)
            .then(|| cursor.read());

        let item_type = reason
            .contains(DeathReason::HAS_ITEM_TY)
            .then(|| cursor.read());

        let item_prefix = reason
            .contains(DeathReason::HAS_ITEM_PREFIX)
            .then(|| cursor.read());

        let death_reason = reason
            .contains(DeathReason::HAS_CUSTOM_MSG)
            .then(|| cursor.read());

        Self {
            killer_player_id,
            killing_npc_index,
            projectile_index,
            type_of_death,
            projectile_type,
            item_type,
            item_prefix,
            death_reason,
        }
    }
}
