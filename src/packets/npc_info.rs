use crate::packets::{PacketBody, Vec2};
use crate::serialization::SliceCursor;

/// Information about a non-playable character in the world.
#[derive(Debug)]
pub struct NpcInfo {
    pub id: u16,
    pub pos: Vec2,
    pub vel: Vec2,
    pub target: u16,
    pub life: i32,
    pub flags: u16,
    pub ai: Vec<f32>, // up to 4, depends on flags 0..4
    pub net_id: i32,
    pub stats_scaled: u8,         // depends on flag
    pub strength_multiplier: f32, // depends on flag
    pub life_flags: u8,
    pub extra_life: i32, // ???
    pub owner: u8,
}

impl PacketBody for NpcInfo {
    const TAG: u8 = 23;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.id);
        cursor.write(&self.pos);
        cursor.write(&self.vel);
        // TODO rest of fields
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            id: cursor.read(),
            pos: cursor.read(),
            vel: cursor.read(),
            target: Default::default(),
            life: Default::default(),
            flags: Default::default(),
            ai: Default::default(),
            net_id: Default::default(),
            stats_scaled: Default::default(),
            strength_multiplier: Default::default(),
            life_flags: Default::default(),
            extra_life: Default::default(),
            owner: Default::default(),
        }
    }
}
