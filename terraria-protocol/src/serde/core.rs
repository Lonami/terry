//! Implements Serializable and Deserializable for "core" types.

use crate::serde::{Deserializable, Error, Result, Serializable, SliceCursor};

use std::convert::TryInto;

macro_rules! impl_serde_int {
    ($read:ident: $($ty:ty),+) => {
        $(
            impl Serializable for $ty {
                fn serialize(&self, cursor: &mut SliceCursor) -> Result<()> {
                    cursor.write_slice(&self.to_le_bytes())
                }
            }

            impl Deserializable for $ty {
                fn deserialize(cursor: &mut SliceCursor) -> Result<Self> {
                    cursor.$read().map(Self::from_le_bytes)
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
    fn serialize(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&(*self as u8))
    }
}

impl Deserializable for bool {
    fn deserialize(cursor: &mut SliceCursor) -> Result<Self> {
        cursor.read::<u8>().map(|b| b != 0)
    }
}

impl Serializable for String {
    fn serialize(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write::<u8>(&self.len().try_into().map_err(|_| Error::PrematureEnd)?)?;
        cursor.write_slice(self.as_bytes())
    }
}

impl Deserializable for String {
    fn deserialize(cursor: &mut SliceCursor) -> Result<Self> {
        let len = u8::deserialize(cursor)? as usize;
        Ok(String::from_utf8_lossy(cursor.readn(len)?).to_string())
    }
}
