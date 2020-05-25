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
}

pub trait Serializable {
    fn serialize(&self, cursor: &mut SliceCursor);
}

pub trait Deserializable {
    fn deserialize(cursor: &mut SliceCursor) -> Self;
}
