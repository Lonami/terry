use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Send section.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct SendSection {
    pub compressed: bool,
    pub x_start: i32,
    pub y_start: i32,
    pub width: i16,
    pub height: i16,
    pub tiles: (),
    pub chest_count: i16,
    pub chests: (),
    pub sign_count: i16,
    pub signs: (),
    pub tileentity_count: i16,
    pub tileentities: (),
}

impl PacketBody for SendSection {
    const TAG: u8 = 10;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
        /*
        cursor.write(&self.compressed);
        cursor.write(&self.x_start);
        cursor.write(&self.y_start);
        cursor.write(&self.width);
        cursor.write(&self.height);
        cursor.write(&self.tiles);
        cursor.write(&self.chest_count);
        cursor.write(&self.chests);
        cursor.write(&self.sign_count);
        cursor.write(&self.signs);
        cursor.write(&self.tileentity_count);
        cursor.write(&self.tileentities);
        */
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        todo!()
        /*
        Self {
            compressed: cursor.read(),
            x_start: cursor.read(),
            y_start: cursor.read(),
            width: cursor.read(),
            height: cursor.read(),
            tiles: cursor.read(),
            chest_count: cursor.read(),
            chests: cursor.read(),
            sign_count: cursor.read(),
            signs: cursor.read(),
            tileentity_count: cursor.read(),
            tileentities: cursor.read(),
        }
        */
    }
}
