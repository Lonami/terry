use std::convert::TryInto;

pub struct SliceCursor<'a> {
    slice: &'a mut [u8],
    pos: usize,
}

impl SliceCursor<'_> {
    #[inline(always)]
    fn read<D: Deserializable>(&mut self) -> D {
        D::deserialize(self)
    }

    #[inline(always)]
    fn write<S: Serializable>(&mut self, s: &S) {
        s.serialize(self)
    }

    #[inline(always)]
    fn rewrite<S: Serializable>(&mut self, pos: usize, s: &S) {
         let last = self.pos;
         self.pos = pos;
         self.write(s);
         assert!(self.pos <= last);
         self.pos = last;
    }

    #[inline(always)]
    fn pos(&self) -> usize {
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
        [self.slice[self.pos - 4], self.slice[self.pos - 3], self.slice[self.pos - 2], self.slice[self.pos - 1]]
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

impl Serializable for u8 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write_slice(&[*self]);
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

impl Deserializable for u8 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        u8::from_le_bytes(cursor.read1())
    }
}

impl Deserializable for u16 {
    fn deserialize( cursor: &mut SliceCursor) -> Self {
        u16::from_le_bytes(cursor.read2())
    }
}

impl Deserializable for u32 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        u32::from_le_bytes(cursor.read4())
    }
}

impl Deserializable for String {
    fn deserialize( cursor: &mut SliceCursor) -> Self {
        let len = u8::deserialize(cursor) as usize;
        String::from_utf8_lossy(cursor.readn(len)).to_string()
    }
}
