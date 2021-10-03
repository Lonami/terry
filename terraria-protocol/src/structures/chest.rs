use crate::{Deserializable, Serializable, SliceCursor};

#[derive(Debug)]
pub struct Chest {
    pub index: u16,
    pub x: i16,
    pub y: i16,
    pub name: String,
}

impl Serializable for Chest {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.index);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.name);
    }
}

impl Deserializable for Chest {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self {
            index: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            name: cursor.read(),
        }
    }
}
