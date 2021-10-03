use crate::serde::{serializable_enum, Deserializable, Result, Serializable, SliceCursor};
use std::convert::TryInto;

serializable_enum! {
    pub enum NetStringMode: u8 {
        Literal = 0,
        Formattable = 1,
        LocalizationKey = 2,
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct NetString {
    mode: NetStringMode,
    text: String,
    substitutions: Vec<NetString>,
}

impl NetString {
    pub fn len(&self) -> usize {
        self.text.len()
    }
}

impl Serializable for NetString {
    fn serialize(&self, cursor: &mut SliceCursor) -> Result<()> {
        cursor.write(&self.mode)?;
        cursor.write(&self.text)?;
        if self.mode != NetStringMode::Literal {
            let len: u8 = self
                .substitutions
                .len()
                .try_into()
                .expect("too many substitutions");
            cursor.write(&len)?;
            for s in self.substitutions.iter() {
                cursor.write(s)?;
            }
        }
        Ok(())
    }
}

impl Deserializable for NetString {
    fn deserialize(cursor: &mut SliceCursor) -> Result<Self> {
        let mode = cursor.read()?;
        let text = cursor.read()?;
        let mut substitutions = Vec::new();
        if mode != NetStringMode::Literal {
            let len = cursor.read::<u8>()? as usize;
            substitutions.reserve(len);
            for _ in 0..len {
                substitutions.push(cursor.read()?);
            }
        }
        Ok(Self {
            mode,
            text,
            substitutions,
        })
    }
}
