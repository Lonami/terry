use crate::packets::PacketBody;
use crate::SliceCursor;

/// Wipe everything in the Eternia Crystal Invasion.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct CrystalInvasionWipe {}

impl PacketBody for CrystalInvasionWipe {
    const TAG: u8 = 114;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
