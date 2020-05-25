use crate::{Deserializable, Serializable, SliceCursor};

#[derive(Debug)]
pub struct Tile {
    pub flags: [u8; 2],
    pub color: Option<u8>,
    pub wall_color: Option<u8>,
    pub ty: Option<u16>,
    pub frame_x: Option<u16>,
    pub frame_y: Option<u16>,
    pub wall: Option<u16>,
    pub liquid: Option<u8>,
    pub liquid_type: Option<u8>,
}

impl Tile {
    fn active(&self) -> bool {
        self.flags[0] & 0x01 != 0
    }
    fn lighted(&self) -> bool {
        self.flags[0] & 0x02 != 0
    }
    fn has_wall(&self) -> bool {
        self.flags[0] & 0x04 != 0
    }
    fn has_liquid(&self) -> bool {
        self.flags[0] & 0x08 != 0
    }
    fn wire1(&self) -> bool {
        self.flags[0] & 0x10 != 0
    }
    fn half_brick(&self) -> bool {
        self.flags[0] & 0x20 != 0
    }
    fn actuator(&self) -> bool {
        self.flags[0] & 0x40 != 0
    }
    fn inactive(&self) -> bool {
        self.flags[0] & 0x80 != 0
    }

    fn wire2(&self) -> bool {
        self.flags[1] & 0x01 != 0
    }
    fn wire3(&self) -> bool {
        self.flags[1] & 0x02 != 0
    }
    fn has_color(&self) -> bool {
        self.flags[1] & 0x04 != 0
    }
    fn has_wall_color(&self) -> bool {
        self.flags[1] & 0x08 != 0
    }
    fn slope1(&self) -> bool {
        self.flags[1] & 0x10 != 0
    }
    fn slope2(&self) -> bool {
        self.flags[1] & 0x20 != 0
    }
    fn slope3(&self) -> bool {
        self.flags[1] & 0x40 != 0
    }
    fn wire4(&self) -> bool {
        self.flags[1] & 0x80 != 0
    }
}

impl Serializable for Tile {
    fn serialize(&self, cursor: &mut SliceCursor) {
        // TODO yes we need a better way to deal with bitflags and options
        self.flags.iter().for_each(|f| cursor.write(f));
        if self.has_color() {
            cursor.write(&self.color.unwrap());
        }
        if self.has_wall_color() {
            cursor.write(&self.wall_color.unwrap());
        }
        if self.active() {
            cursor.write(&self.ty.unwrap());
        }
        let tile_frame_important = true; // ???
        if self.active() && tile_frame_important {
            cursor.write(&self.frame_x.unwrap());
            cursor.write(&self.frame_y.unwrap());
        }
        if self.has_wall() {
            cursor.write(&self.wall.unwrap());
        }
        if self.has_liquid() {
            cursor.write(&self.liquid.unwrap());
            cursor.write(&self.liquid_type.unwrap());
        }
    }
}

impl Deserializable for Tile {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        todo!()
    }
}
