use crate::serde::SliceCursor;
use crate::Packet;

#[derive(Debug)]
pub struct Parser {
    buffer: Vec<u8>,
}

impl Parser {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn feed(&mut self, data: &[u8]) {
        self.buffer.extend(data);
    }

    pub fn next(&mut self) -> Option<Packet> {
        if self.buffer.len() < 2 {
            return None;
        }

        let mut lenbuf = [self.buffer[0], self.buffer[1]];
        let mut cursor = SliceCursor::new(&mut lenbuf);
        let len = cursor.read::<u16>().ok()? as usize;
        if self.buffer.len() < len {
            return None;
        }

        let mut cursor = SliceCursor::new(&mut self.buffer);
        let packet = cursor.read().ok()?;
        self.buffer.copy_within(len.., 0);
        self.buffer.truncate(self.buffer.len() - len);
        return Some(packet);
    }
}
