use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// WeaponsRackTryPlacing.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct WeaponsRackTryPlacing {
    pub x: i16,
    pub y: i16,
    pub net_id: i16,
    pub prefix: u8,
    pub stack: i16,
}

impl PacketBody for WeaponsRackTryPlacing {
    const TAG: u8 = 123;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.net_id);
        cursor.write(&self.prefix);
        cursor.write(&self.stack);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
            net_id: cursor.read(),
            prefix: cursor.read(),
            stack: cursor.read(),
        }
    }
}
