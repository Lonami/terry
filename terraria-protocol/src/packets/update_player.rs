use crate::packets::PacketBody;
use crate::SliceCursor;

/// Update Player.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdatePlayer {
    pub player_id: u8,
    // bit flags {
    pub key_up: bool,
    pub key_down: bool,
    pub key_left: bool,
    pub key_right: bool,
    pub key_jump: bool,
    pub key_item_use: bool,
    pub facing_right: bool,
    // }
    // bit flags {
    pub pulley: bool,
    pub pulley_right: bool,
    pub update_velocity: bool,
    pub vortex_stealth: bool,
    pub gravity_flipped: bool,
    pub shield_raised: bool,
    // }
    // bit flags {
    pub hovering_up: bool,
    pub void_vault: bool,
    pub sitting: bool,
    pub downed_dd2: bool,
    pub petting_animal: bool,
    pub petting_small_animal: bool,
    pub used_return_potion: bool,
    pub hovering_down: bool,
    // }
    // bit flags {
    pub sleeping: u8,
    // }
    pub selected_item: u8,
    pub position_x: i32, /* single */
    pub position_y: i32, /* single */
    /// Not sent if UpdateVelocity is not set
    pub velocity_x: i32, /* single */
    /// Not sent if UpdateVelocity is not set
    pub velocity_y: i32, /* single */
    /// Original Position for Potion of Return, only sent if UsedPotionofReturn flag is true
    pub original_position_x: i32, /* single */
    /// Original Position for Potion of Return, only sent if UsedPotionofReturn flag is true
    pub original_position_y: i32, /* single */
    /// Home Position for Potion of Return, only sent if UsedPotionofReturn flag is true
    pub home_position_x: i32, /* single */
    /// Home Position for Potion of Return, only sent if UsedPotionofReturn flag is true
    pub home_position_y: i32, /* single */
}

impl PacketBody for UpdatePlayer {
    const TAG: u8 = 13;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.control);
        cursor.write(&self.pulley);
        cursor.write(&self.misc);
        cursor.write(&self.sleepinginfo);
        cursor.write(&self.selected_item);
        cursor.write(&self.position_x);
        cursor.write(&self.position_y);
        cursor.write(&self.velocity_x);
        cursor.write(&self.velocity_y);
        cursor.write(&self.original_position_x);
        cursor.write(&self.original_position_y);
        cursor.write(&self.home_position_x);
        cursor.write(&self.home_position_y);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            control: cursor.read(),
            pulley: cursor.read(),
            misc: cursor.read(),
            sleepinginfo: cursor.read(),
            selected_item: cursor.read(),
            position_x: cursor.read(),
            position_y: cursor.read(),
            velocity_x: cursor.read(),
            velocity_y: cursor.read(),
            original_position_x: cursor.read(),
            original_position_y: cursor.read(),
            home_position_x: cursor.read(),
            home_position_y: cursor.read(),
        }
    }
}
