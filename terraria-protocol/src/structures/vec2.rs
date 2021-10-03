use crate::structures::serializable_struct;

pub const TILE_TO_POS_SCALE: f32 = 16.0;

serializable_struct! {
    pub struct Vec2 {
        pub x: f32,
        pub y: f32,
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn from_tile_pos(x: i16, y: i16) -> Self {
        Vec2 {
            x: (x as f32) * TILE_TO_POS_SCALE,
            // Players seem to be 1.625 tiles tall and we need to be above
            y: (y as f32 - 2.625) * TILE_TO_POS_SCALE,
        }
    }
}
