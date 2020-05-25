use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Growth sound effects.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct GrowFx {
    /// 1 = Tree Growth Effects, 2 = Fairy Effects
    pub effectflags: u8,
    pub x: i32,
    pub y: i32,
    pub height: u8,
    pub tree_gore: i16,
}

impl PacketBody for GrowFx {
    const TAG: u8 = 112;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.effectflags);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.height);
        cursor.write(&self.tree_gore);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            effectflags: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            height: cursor.read(),
            tree_gore: cursor.read(),
        }
    }
}
