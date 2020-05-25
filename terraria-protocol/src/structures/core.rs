use crate::{Deserializable, Serializable, SliceCursor};
use std::convert::TryInto;

impl Serializable for bool {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
    }
}

impl Serializable for i8 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&self.to_le_bytes());
    }
}

impl Serializable for u8 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&self.to_le_bytes());
    }
}

impl Serializable for i16 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&self.to_le_bytes());
    }
}

impl Serializable for u16 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&self.to_le_bytes());
    }
}

impl Serializable for i32 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&self.to_le_bytes());
    }
}

impl Serializable for u32 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&self.to_le_bytes());
    }
}

impl Serializable for f32 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&self.to_le_bytes());
    }
}

impl Serializable for i64 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&self.to_le_bytes());
    }
}

impl Serializable for u64 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&self.to_le_bytes());
    }
}

impl Serializable for String {
    fn serialize(&self, cursor: &mut SliceCursor) {
        let len: u8 = self.len().try_into().expect("string too long");
        cursor.write(&len);
        cursor.write_slice(self.as_bytes());
    }
}

impl Deserializable for bool {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        cursor.read::<u8>() != 0
    }
}

impl Deserializable for i8 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_le_bytes(cursor.read1())
    }
}

impl Deserializable for u8 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_le_bytes(cursor.read1())
    }
}

impl Deserializable for i16 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_le_bytes(cursor.read2())
    }
}

impl Deserializable for u16 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_le_bytes(cursor.read2())
    }
}

impl Deserializable for i32 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_le_bytes(cursor.read4())
    }
}

impl Deserializable for u32 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_le_bytes(cursor.read4())
    }
}

impl Deserializable for f32 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_le_bytes(cursor.read4())
    }
}

impl Deserializable for i64 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_le_bytes(cursor.read8())
    }
}

impl Deserializable for u64 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self::from_le_bytes(cursor.read8())
    }
}

impl Deserializable for String {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        let len = u8::deserialize(cursor) as usize;
        String::from_utf8_lossy(cursor.readn(len)).to_string()
    }
}
