use crate::serde::{serializable_bitflags, PacketBody, Result, SliceCursor};
use crate::structures::Vec2;

use std::convert::TryInto;
use std::mem::size_of;

serializable_bitflags! {
    pub struct NpcFlag: u16 {
        const HORIZONTAL_DIR = 0x0001;
        const VERTICAL_DIR = 0x0002;
        const AI1 = 0x0004;
        const AI2 = 0x0008;
        const AI3 = 0x0010;
        const AI4 = 0x0020;
        const SPRITE_DIR = 0x0040;
        const HAS_LIFE = 0x0080;
        const SCALE_PLAYER_COUNT = 0x0100;
        const SPAWNED_FROM_STATUE = 0x0200;
        const MULTIPLY_STRENGTH = 0x0400;
    }
}

/// NPC update, such as movement.
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct NpcUpdate {
    pub npc_id: i16,
    pub pos: Vec2,
    pub vel: Vec2,
    /// Player ID
    pub target: u16,
    pub flags: NpcFlag,
    /// Only has meaningful values if the corresponding flag is set
    pub ai: [f32; 4],
    pub npc_net_id: i16,
    /// Player count needed to set the multiplayer difficulty override
    pub player_count_scale: Option<u8>,
    pub strength_multiplier: Option<f32>,
    /// If not present, it means the life is at its maximum
    pub life: Option<i32>,
    /// Only present if catchable
    pub release_owner: Option<u8>,
}

impl Eq for NpcUpdate {}

// 1 if it's catchable, 0 otherwise
const NPC_CATCHABLE: [u8; 668] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
];

impl PacketBody for NpcUpdate {
    const TAG: u8 = 23;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.npc_id)?;
        cursor.write(&self.pos)?;
        cursor.write(&self.vel)?;
        cursor.write(&self.target)?;
        cursor.write(&self.flags)?;
        for (i, _) in [NpcFlag::AI1, NpcFlag::AI2, NpcFlag::AI3, NpcFlag::AI4]
            .iter()
            .enumerate()
            .filter(|(_, flag)| self.flags.contains(**flag))
        {
            cursor.write(&self.ai[i])?;
        }
        cursor.write(&self.npc_net_id)?;
        if let Some(scale) = self.player_count_scale.as_ref() {
            cursor.write(scale)?;
        }
        if let Some(mult) = self.strength_multiplier.as_ref() {
            cursor.write(mult)?;
        }
        if let Some(life) = self.life {
            if let Ok(hp) = life.try_into() {
                cursor.write(&(size_of::<i8>() as u8));
                cursor.write::<i8>(&hp);
            } else if let Ok(hp) = life.try_into() {
                cursor.write(&(size_of::<i16>() as u8));
                cursor.write::<i16>(&hp);
            } else {
                cursor.write(&(size_of::<i32>() as u8));
                cursor.write::<i32>(&life);
            }
        }
        if let Some(release_owner) = self.release_owner {
            cursor.write(&release_owner)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        let npc_id = cursor.read()?;
        let pos = cursor.read()?;
        let vel = cursor.read()?;
        let target = cursor.read()?;
        let flags = cursor.read::<NpcFlag>()?;

        let mut ai = [0f32; 4];
        for (i, _) in [NpcFlag::AI1, NpcFlag::AI2, NpcFlag::AI3, NpcFlag::AI4]
            .iter()
            .enumerate()
            .filter(|(_, flag)| flags.contains(**flag))
        {
            ai[i] = cursor.read()?;
        }

        let npc_net_id = cursor.read()?;

        let player_count_scale = flags
            .contains(NpcFlag::SCALE_PLAYER_COUNT)
            .then(|| cursor.read())
            .transpose()?;

        let strength_multiplier = flags
            .contains(NpcFlag::MULTIPLY_STRENGTH)
            .then(|| cursor.read())
            .transpose()?;

        let life = flags
            .contains(NpcFlag::HAS_LIFE)
            .then(|| {
                Ok(match cursor.read::<u8>()? {
                    1 => cursor.read::<i8>()? as i32,
                    2 => cursor.read::<i16>()? as i32,
                    4 => cursor.read::<i32>()? as i32,
                    n => panic!("bad byte count for life {}", n),
                })
            })
            .transpose()?;

        let release_owner = (npc_net_id >= 0 && NPC_CATCHABLE[npc_net_id as usize] != 0)
            .then(|| cursor.read())
            .transpose()?;

        Ok(Self {
            npc_id,
            pos,
            vel,
            target,
            flags,
            ai,
            npc_net_id,
            player_count_scale,
            strength_multiplier,
            life,
            release_owner,
        })
    }
}
