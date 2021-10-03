mod core;

use std::convert::TryInto as _;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    /// Reached a premature end of data, and the type could not be deseariled as a whole.
    PrematureEnd,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PrematureEnd => write!(f, "deserialization error: premature end of data"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub struct SliceCursor<'a> {
    slice: &'a mut [u8],
    pos: usize,
}

impl<'a> SliceCursor<'a> {
    pub fn new(slice: &'a mut [u8]) -> Self {
        Self { slice, pos: 0 }
    }

    #[inline(always)]
    pub fn read<D: Deserializable>(&mut self) -> Result<D> {
        D::deserialize(self)
    }

    #[inline(always)]
    pub fn write<S: Serializable>(&mut self, s: &S) -> Result<()> {
        s.serialize(self)
    }

    #[inline(always)]
    pub fn rewrite<S: Serializable>(&mut self, pos: usize, s: &S) -> Result<()> {
        let last = self.pos;
        self.pos = pos;
        self.write(s)?;
        assert!(self.pos <= last);
        self.pos = last;
        Ok(())
    }

    #[inline(always)]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline(always)]
    pub fn eof(&self) -> bool {
        self.pos == self.slice.len()
    }

    #[inline(always)]
    pub fn finish(self) -> (&'a mut [u8], usize) {
        (self.slice, self.pos)
    }

    // If the `const` feautre was a bit more advanced we could probably make
    // this return `[u8; n]` which would play nice with `{integer}::from_le`.
    // TODO don't panic!() in these
    #[inline(always)]
    pub(crate) fn readn(&mut self, n: usize) -> Result<&[u8]> {
        self.pos += n;
        Ok(&self.slice[self.pos - n..self.pos])
    }

    #[inline(always)]
    pub(crate) fn read1(&mut self) -> Result<[u8; 1]> {
        self.pos += 1;
        Ok([self.slice[self.pos - 1]])
    }

    #[inline(always)]
    pub(crate) fn read2(&mut self) -> Result<[u8; 2]> {
        self.pos += 2;
        Ok([self.slice[self.pos - 2], self.slice[self.pos - 1]])
    }

    #[inline(always)]
    pub(crate) fn read4(&mut self) -> Result<[u8; 4]> {
        self.pos += 4;
        Ok([
            self.slice[self.pos - 4],
            self.slice[self.pos - 3],
            self.slice[self.pos - 2],
            self.slice[self.pos - 1],
        ])
    }

    #[inline(always)]
    pub(crate) fn read8(&mut self) -> Result<[u8; 8]> {
        self.pos += 8;
        Ok([
            self.slice[self.pos - 8],
            self.slice[self.pos - 7],
            self.slice[self.pos - 6],
            self.slice[self.pos - 5],
            self.slice[self.pos - 4],
            self.slice[self.pos - 3],
            self.slice[self.pos - 2],
            self.slice[self.pos - 1],
        ])
    }

    #[inline(always)]
    pub(crate) fn write_slice(&mut self, slice: &[u8]) -> Result<()> {
        let pos = self.pos;
        self.slice[pos..pos + slice.len()].copy_from_slice(slice);
        self.pos += slice.len();
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn read_to_end(&mut self) -> &[u8] {
        let pos = self.pos;
        self.pos = self.slice.len();
        &self.slice[pos..]
    }
}

pub trait Serializable {
    fn serialize(&self, cursor: &mut SliceCursor) -> Result<()>;
}

pub trait Deserializable {
    fn deserialize(cursor: &mut SliceCursor) -> Result<Self>
    where
        Self: Sized;
}

pub trait PacketBody: Sized {
    const TAG: u8;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()>;

    fn from_body(cursor: &mut SliceCursor) -> Result<Self>;

    // TODO player should probably go inside the packets
    fn serialize_as_packet(&self, cursor: &mut SliceCursor) -> Result<()> {
        let length_pos = cursor.pos();
        cursor.write(&0u16)?; // length
        cursor.write(&Self::TAG)?;
        self.write_body(cursor)?;
        let length: u16 = (cursor.pos() - length_pos)
            .try_into()
            .expect("packet too long");
        cursor.rewrite(length_pos, &length)?;
        Ok(())
    }
}

macro_rules! serializable_struct {
    (
        $(#[$attr:meta])*
        pub struct $ident:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident: $ty:ty,
            )*
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, PartialEq, Default, Clone)]
        pub struct $ident {
            $(
                $(#[$field_attr])*
                pub $field: $ty,
            )+
        }

        impl Eq for $ident {}

        impl crate::serde::Serializable for $ident {
            fn serialize(&self, cursor: &mut crate::serde::SliceCursor) -> crate::serde::Result<()> {
                $(cursor.write(&self.$field)?;)+
                Ok(())
            }
        }

        impl crate::serde::Deserializable for $ident {
            fn deserialize(cursor: &mut crate::serde::SliceCursor) -> crate::serde::Result<Self> {
                Ok(Self {
                    $($field: cursor.read()?,)+
                })
            }
        }
    };
}

macro_rules! serializable_enum {
    (
        pub enum $ident:ident: $ty:ident {
            $first_variant:ident = $first_value:expr,
            $($variant:ident = $value:expr,)*
        }
    ) => {
        #[repr($ty)]
        #[derive(PartialEq, Eq, Copy, Clone, Debug)]
        pub enum $ident {
            $first_variant = $first_value,
            $($variant = $value,)*
        }

        impl Default for $ident {
            fn default() -> Self {
                Self::$first_variant
            }
        }

        impl crate::serde::Serializable for $ident {
            fn serialize(&self, cursor: &mut crate::serde::SliceCursor) -> crate::serde::Result<()> {
                cursor.write(&(*self as $ty))
            }
        }

        impl crate::serde::Deserializable for $ident {
            fn deserialize(cursor: &mut crate::serde::SliceCursor) -> crate::serde::Result<Self> {
                Ok(match cursor.read::<$ty>()? {
                    $first_value => $ident::$first_variant,
                    $($value => $ident::$variant,)*
                    n => panic!("invalid {}: {}", stringify!($ty), n),
                })
            }
        }
    };
}

macro_rules! serializable_bitflags {
    (
        pub struct $ident:ident: $ty:ident {
            $(const $variant:ident = $value:expr;)+
        }
    ) => {
        bitflags::bitflags! {
            pub struct $ident: $ty {
                $(const $variant = $value;)+
            }
        }

        impl Default for $ident {
            fn default() -> Self {
                Self::empty()
            }
        }

        impl crate::serde::Serializable for $ident {
            fn serialize(&self, cursor: &mut crate::serde::SliceCursor) -> crate::serde::Result<()> {
                cursor.write(&self.bits())
            }
        }

        impl crate::serde::Deserializable for $ident {
            fn deserialize(cursor: &mut crate::serde::SliceCursor) -> crate::serde::Result<Self> {
                cursor.read().map(Self::from_bits_truncate)
            }
        }
    };
}

macro_rules! packet_struct {
    (
        $(#[$attr:meta])*
        pub struct $ident:ident {
            const TAG = $tag:expr;

            $(
                $(#[$field_attr:meta])*
                pub $field:ident: $ty:ty,
            )*
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, PartialEq, Default, Clone)]
        pub struct $ident {
            $(
                $(#[$field_attr])*
                pub $field: $ty,
            )*
        }

        impl Eq for $ident {}

        impl crate::serde::PacketBody for $ident {
            const TAG: u8 = $tag;

            fn write_body(&self, cursor: &mut crate::serde::SliceCursor) -> crate::serde::Result<()> {
                let _ = cursor;
                $(cursor.write(&self.$field)?;)*
                Ok(())
            }

            fn from_body(cursor: &mut crate::serde::SliceCursor) -> crate::serde::Result<Self> {
                let _ = cursor;
                Ok(Self {
                    $($field: cursor.read()?,)*
                })
            }
        }
    };
}

pub(crate) use {packet_struct, serializable_bitflags, serializable_enum, serializable_struct};
