use crate::{Deserializable, Serializable, SliceCursor};
use std::convert::TryInto;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NetStringMode {
    Literal = 0,
    Formattable = 1,
    LocalizationKey = 2,
}

impl Serializable for NetStringMode {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
    }
}

impl Deserializable for NetStringMode {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        match cursor.read::<u8>() {
            0 => NetStringMode::Literal,
            1 => NetStringMode::Formattable,
            2 => NetStringMode::LocalizationKey,
            n => panic!(format!("invalid net string mode {}", n)),
        }
    }
}

#[derive(Debug)]
pub struct NetString {
    mode: NetStringMode,
    text: String,
    substitutions: Vec<NetString>,
}

impl Serializable for NetString {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.mode);
        cursor.write(&self.text);
        if self.mode != NetStringMode::Literal {
            let len: u8 = self
                .substitutions
                .len()
                .try_into()
                .expect("too many substitutions");
            cursor.write(&len);
            self.substitutions.iter().for_each(|s| cursor.write(s));
        }
    }
}

impl Deserializable for NetString {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        let mode = cursor.read();
        let text = cursor.read();
        let mut substitutions = Vec::new();
        if mode != NetStringMode::Literal {
            let len = cursor.read::<u8>() as usize;
            substitutions.reserve(len);
            (0..len).for_each(|_| substitutions.push(cursor.read()));
        }
        Self {
            mode,
            text,
            substitutions,
        }
    }
}
