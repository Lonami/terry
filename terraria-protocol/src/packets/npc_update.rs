use crate::packets::PacketBody;
use crate::structures::Vec2;
use crate::SliceCursor;
use std::convert::TryInto;
use std::mem::size_of;

/// NPC update, such as movement.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct NpcUpdate {
    pub npc_id: i16,
    pub pos: Vec2,
    pub vel: Vec2,
    /// Player ID.
    pub target: u16,
    // bitflags {
    pub horizontal_dir: bool,
    pub vertical_dir: bool,
    pub sprite_dir: bool,
    // }
    // bitflags {
    pub spawned_from_statue: bool,
    // }
    /// Can have up to 4 values.
    pub ai: Vec<f32>,
    pub npc_net_id: i16,
    /// Player count needed to set the multiplayer difficulty override.
    pub player_count_scale: Option<u8>,
    pub strength_multiplier: Option<f32>,
    /// If not present, it means the life is at its maximum.
    pub life: Option<i32>,
    // TODO which NPCs are catchable?
    // Only present if NPC is catchable.
    // pub release_owner: u8,
}

impl PacketBody for NpcUpdate {
    const TAG: u8 = 23;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.pos);
        cursor.write(&self.vel);
        cursor.write(&self.target);
        cursor.write(
            &(0 // npc flags[0]
            | if self.horizontal_dir { 0x01 } else { 0 }
            | if self.vertical_dir { 0x02 } else { 0 }
            | if self.ai.len() >= 1 { 0x04 } else { 0 }
            | if self.ai.len() >= 2 { 0x08 } else { 0 }
            | if self.ai.len() >= 3 { 0x10 } else { 0 }
            | if self.ai.len() >= 4 { 0x20 } else { 0 }
            | if self.sprite_dir { 0x40 } else { 0 }
            | if self.life.is_none() { 0x80 } else { 0 }),
        );
        cursor.write(
            &(0 // npc flags[1]
            | if self.player_count_scale.is_some() { 0x01 } else { 0 }
            | if self.spawned_from_statue { 0x02 } else { 0 }
            | if self.strength_multiplier.is_some() { 0x04 } else { 0 }),
        );
        self.ai.iter().take(4).for_each(|a| cursor.write(a));
        cursor.write(&self.npc_net_id);
        if let Some(scale) = self.player_count_scale.as_ref() {
            cursor.write(scale);
        }
        if let Some(mult) = self.strength_multiplier.as_ref() {
            cursor.write(mult);
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
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let npc_id = cursor.read();
        let pos = cursor.read();
        let vel = cursor.read();
        let target = cursor.read();
        let npc_flags: [u8; 2] = [cursor.read(), cursor.read()];
        let mut ai = Vec::with_capacity(4);
        if npc_flags[0] & 0x04 != 0 {
            ai.push(cursor.read());
        }
        if npc_flags[0] & 0x08 != 0 {
            ai.push(cursor.read());
        }
        if npc_flags[0] & 0x10 != 0 {
            ai.push(cursor.read());
        }
        if npc_flags[0] & 0x20 != 0 {
            ai.push(cursor.read());
        }
        let npc_net_id = cursor.read();
        let player_count_scale = if npc_flags[1] & 0x01 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let strength_multiplier = if npc_flags[1] & 0x04 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let life = if npc_flags[0] & 0x80 == 0 {
            Some(match cursor.read::<u8>() {
                1 => cursor.read::<i8>() as i32,
                2 => cursor.read::<i16>() as i32,
                4 => cursor.read::<i32>() as i32,
                n => panic!("bad byte count for life {}", n),
            })
        } else {
            None
        };

        Self {
            npc_id,
            pos,
            vel,
            target,
            horizontal_dir: npc_flags[0] & 0x01 != 0,
            vertical_dir: npc_flags[0] & 0x02 != 0,
            sprite_dir: npc_flags[0] & 0x40 != 0,
            spawned_from_statue: npc_flags[1] & 0x02 != 0,
            ai,
            npc_net_id,
            player_count_scale,
            strength_multiplier,
            life,
        }
    }
}
