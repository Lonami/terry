use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// A compressed region of tiles.
#[derive(Debug)]
pub struct CompressedTileBlock {
    // TODO actual compression and decompression
}

impl PacketBody for CompressedTileBlock {
    const TAG: u8 = 10;

    fn write_body(&self, _cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
