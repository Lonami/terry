use crate::{Deserializable, Serializable, SliceCursor};
use std::convert::TryInto;

macro_rules! impl_serde_int {
    ($read:ident: $($ty:ty),+) => {
        $(
            impl Serializable for $ty {
                fn serialize(&self, cursor: &mut SliceCursor) {
                    cursor.write_slice(&self.to_le_bytes());
                }
            }

            impl Deserializable for $ty {
                fn deserialize(cursor: &mut SliceCursor) -> Self {
                    Self::from_le_bytes(cursor.$read())
                }
            }
        )+
    };
}

impl_serde_int!(read1: i8, u8);
impl_serde_int!(read2: i16, u16);
impl_serde_int!(read4: i32, u32, f32);
impl_serde_int!(read8: i64, u64);

impl Serializable for bool {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
    }
}

impl Deserializable for bool {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        cursor.read::<u8>() != 0
    }
}

impl Serializable for String {
    fn serialize(&self, cursor: &mut SliceCursor) {
        let len: u8 = self.len().try_into().expect("string too long");
        cursor.write(&len);
        cursor.write_slice(self.as_bytes());
    }
}

impl Deserializable for String {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        let len = u8::deserialize(cursor) as usize;
        String::from_utf8_lossy(cursor.readn(len)).to_string()
    }
}
