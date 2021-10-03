pub struct SliceCursor<'a> {
    slice: &'a mut [u8],
    pos: usize,
}

impl<'a> SliceCursor<'a> {
    pub fn new(slice: &'a mut [u8]) -> Self {
        Self { slice, pos: 0 }
    }

    #[inline(always)]
    pub fn read<D: Deserializable>(&mut self) -> D {
        D::deserialize(self)
    }

    #[inline(always)]
    pub fn write<S: Serializable>(&mut self, s: &S) {
        s.serialize(self)
    }

    #[inline(always)]
    pub fn rewrite<S: Serializable>(&mut self, pos: usize, s: &S) {
        let last = self.pos;
        self.pos = pos;
        self.write(s);
        assert!(self.pos <= last);
        self.pos = last;
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
    #[inline(always)]
    pub(crate) fn readn(&mut self, n: usize) -> &[u8] {
        self.pos += n;
        &self.slice[self.pos - n..self.pos]
    }

    #[inline(always)]
    pub(crate) fn read1(&mut self) -> [u8; 1] {
        self.pos += 1;
        [self.slice[self.pos - 1]]
    }

    #[inline(always)]
    pub(crate) fn read2(&mut self) -> [u8; 2] {
        self.pos += 2;
        [self.slice[self.pos - 2], self.slice[self.pos - 1]]
    }

    #[inline(always)]
    pub(crate) fn read4(&mut self) -> [u8; 4] {
        self.pos += 4;
        [
            self.slice[self.pos - 4],
            self.slice[self.pos - 3],
            self.slice[self.pos - 2],
            self.slice[self.pos - 1],
        ]
    }

    #[inline(always)]
    pub(crate) fn read8(&mut self) -> [u8; 8] {
        self.pos += 8;
        [
            self.slice[self.pos - 8],
            self.slice[self.pos - 7],
            self.slice[self.pos - 6],
            self.slice[self.pos - 5],
            self.slice[self.pos - 4],
            self.slice[self.pos - 3],
            self.slice[self.pos - 2],
            self.slice[self.pos - 1],
        ]
    }

    #[inline(always)]
    pub(crate) fn write_slice(&mut self, slice: &[u8]) {
        let pos = self.pos;
        self.slice[pos..pos + slice.len()].copy_from_slice(slice);
        self.pos += slice.len();
    }

    #[inline(always)]
    pub(crate) fn read_to_end(&mut self) -> &[u8] {
        let pos = self.pos;
        self.pos = self.slice.len();
        &self.slice[pos..]
    }
}

pub trait Serializable {
    fn serialize(&self, cursor: &mut SliceCursor);
}

pub trait Deserializable {
    fn deserialize(cursor: &mut SliceCursor) -> Self;
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

        impl crate::structures::Serializable for $ident {
            fn serialize(&self, cursor: &mut crate::structures::SliceCursor) {
                $(cursor.write(&self.$field);)+
            }
        }

        impl crate::structures::Deserializable for $ident {
            fn deserialize(cursor: &mut crate::structures::SliceCursor) -> Self {
                Self {
                    $($field: cursor.read(),)+
                }
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

        impl crate::structures::Serializable for $ident {
            fn serialize(&self, cursor: &mut crate::structures::SliceCursor) {
                cursor.write(&(*self as $ty));
            }
        }

        impl crate::structures::Deserializable for $ident {
            fn deserialize(cursor: &mut crate::structures::SliceCursor) -> Self {
                match cursor.read::<$ty>() {
                    $first_value => $ident::$first_variant,
                    $($value => $ident::$variant,)*
                    n => panic!("invalid {}: {}", stringify!($ty), n),
                }
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

        impl crate::structures::Serializable for $ident {
            fn serialize(&self, cursor: &mut crate::structures::SliceCursor) {
                cursor.write(&self.bits());
            }
        }

        impl crate::structures::Deserializable for $ident {
            fn deserialize(cursor: &mut crate::structures::SliceCursor) -> Self {
                Self::from_bits_truncate(cursor.read())
            }
        }
    };
}

pub(crate) use {serializable_bitflags, serializable_enum, serializable_struct};
