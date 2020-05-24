use std::convert::TryInto;

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
    pub fn finish(self) -> usize {
        self.pos
    }

    // If the `const` feautre was a bit more advanced we could probably make
    // this return `[u8; n]` which would play nice with `{integer}::from_le`.
    #[inline(always)]
    fn readn(&mut self, n: usize) -> &[u8] {
        self.pos += n;
        &self.slice[self.pos - n..self.pos]
    }

    #[inline(always)]
    fn read1(&mut self) -> [u8; 1] {
        self.pos += 1;
        [self.slice[self.pos - 1]]
    }

    #[inline(always)]
    fn read2(&mut self) -> [u8; 2] {
        self.pos += 2;
        [self.slice[self.pos - 2], self.slice[self.pos - 1]]
    }

    #[inline(always)]
    fn read4(&mut self) -> [u8; 4] {
        self.pos += 4;
        [
            self.slice[self.pos - 4],
            self.slice[self.pos - 3],
            self.slice[self.pos - 2],
            self.slice[self.pos - 1],
        ]
    }

    #[inline(always)]
    fn write_slice(&mut self, slice: &[u8]) {
        let pos = self.pos;
        self.slice[pos..pos + slice.len()].copy_from_slice(slice);
        self.pos += slice.len();
    }
}

pub trait Serializable {
    fn serialize(&self, cursor: &mut SliceCursor);
}

impl Serializable for bool {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
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

impl Serializable for String {
    fn serialize(&self, cursor: &mut SliceCursor) {
        let len: u8 = self.len().try_into().expect("string too long");
        cursor.write(&len);
        cursor.write_slice(self.as_bytes());
    }
}

pub trait Deserializable {
    fn deserialize(cursor: &mut SliceCursor) -> Self;
}

impl Deserializable for bool {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        cursor.read::<u8>() != 0
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

impl Deserializable for String {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        let len = u8::deserialize(cursor) as usize;
        String::from_utf8_lossy(cursor.readn(len)).to_string()
    }
}
