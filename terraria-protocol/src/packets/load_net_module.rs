use crate::packets::PacketBody;
use crate::SliceCursor;

/// Load a network module.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct LoadNetModule {
    pub module_id: u16,
    pub arguments: Vec<u8>,
}

impl PacketBody for LoadNetModule {
    const TAG: u8 = 82;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.module_id);
        self.arguments.iter().for_each(|b| cursor.write(b));
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let module_id = cursor.read();
        // TODO no idea how to read arguments, we don't have a len
        Self {
            module_id,
            arguments: Vec::new(),
        }
    }
}
