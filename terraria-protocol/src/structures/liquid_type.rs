use crate::{Deserializable, Serializable, SliceCursor};

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LiquidType {
    None = 0,
    Water = 1,
    Lava = 2,
    Honey = 3,
}

impl Serializable for LiquidType {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
    }
}

impl Deserializable for LiquidType {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_value(cursor.read())
    }
}

impl LiquidType {
    pub fn from_value(value: u8) -> Self {
        match value {
            0 => LiquidType::None,
            1 => LiquidType::Water,
            2 => LiquidType::Lava,
            3 => LiquidType::Honey,
            n => panic!("invalid liquid type {}", n),
        }
    }
}
