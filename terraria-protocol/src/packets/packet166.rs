use crate::serde::{PacketBody, Result, SliceCursor};

/// No description known yet.
///
/// Direction: ???.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Packet166 {
    pub buf: Vec<u8>,
}

impl PacketBody for Packet166 {
    const TAG: u8 = 166;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        for b in self.buf.iter() {
            cursor.write(b)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        Ok(Self {
            buf: cursor.read_to_end().to_vec(),
        })
    }
}
