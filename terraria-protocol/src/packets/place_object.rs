use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Place an object.
///
/// Direction: Server <-> Client.
#[derive(Debug)]
pub struct PlaceObject {
    pub x: i16,
    pub y: i16,
    pub ty: i16,
    pub style: i16,
    pub alternate: u8,
    pub random: i8,
    pub direction: bool,
}

impl PacketBody for PlaceObject {
    const TAG: u8 = 79;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.ty);
        cursor.write(&self.style);
        cursor.write(&self.alternate);
        cursor.write(&self.random);
        cursor.write(&self.direction);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            ty: cursor.read(),
            style: cursor.read(),
            alternate: cursor.read(),
            random: cursor.read(),
            direction: cursor.read(),
        }
    }
}
