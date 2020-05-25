use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Mass Wire Operation.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct MassWireOperation {
    pub start_x: i16,
    pub start_y: i16,
    pub end_x: i16,
    pub end_y: i16,
    /// BitFlags: 1 = Red, 2 = Green, 4 = Blue, 8 = Yellow, 16 = Actuator, 32 = Cutter
    pub toolmode: u8,
}

impl PacketBody for MassWireOperation {
    const TAG: u8 = 109;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.start_x);
        cursor.write(&self.start_y);
        cursor.write(&self.end_x);
        cursor.write(&self.end_y);
        cursor.write(&self.toolmode);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            start_x: cursor.read(),
            start_y: cursor.read(),
            end_x: cursor.read(),
            end_y: cursor.read(),
            toolmode: cursor.read(),
        }
    }
}
