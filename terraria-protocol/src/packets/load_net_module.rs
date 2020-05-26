use crate::packets::PacketBody;
use crate::SliceCursor;

/// Load a network module.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct LoadNetModule {
    pub module_id: u16,
    // TODO unknown payload follows
}

impl PacketBody for LoadNetModule {
    const TAG: u8 = 82;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.module_id);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            module_id: cursor.read(),
        }
    }
}
