use crate::{Deserializable, Serializable, SliceCursor};

#[derive(Debug)]
pub struct Sign {
    pub index: u16,
    pub x: i16,
    pub y: i16,
    pub text: String,
}

impl Serializable for Sign {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.index);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.text);
    }
}

impl Deserializable for Sign {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self {
            index: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            text: cursor.read(),
        }
    }
}
