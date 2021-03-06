use crate::packets::PacketBody;
use crate::SliceCursor;

/// Toggle a birthday party.
///
/// Direction: Client -> Server.
#[derive(Debug)]
pub struct ToggleBirthdayParty {}

impl PacketBody for ToggleBirthdayParty {
    const TAG: u8 = 111;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
