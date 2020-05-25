use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update Shield Strengths.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct UpdateShieldStrengths {
    pub solar_tower_shield_strength: u16,
    pub vortex_tower_shield_strength: u16,
    pub nebula_tower_shield_strength: u16,
    pub stardust_tower_shield_strength: u16,
}

impl PacketBody for UpdateShieldStrengths {
    const TAG: u8 = 101;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.solar_tower_shield_strength);
        cursor.write(&self.vortex_tower_shield_strength);
        cursor.write(&self.nebula_tower_shield_strength);
        cursor.write(&self.stardust_tower_shield_strength);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            solar_tower_shield_strength: cursor.read(),
            vortex_tower_shield_strength: cursor.read(),
            nebula_tower_shield_strength: cursor.read(),
            stardust_tower_shield_strength: cursor.read(),
        }
    }
}
