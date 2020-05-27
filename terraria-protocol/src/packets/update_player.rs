use crate::packets::PacketBody;
use crate::structures::Vec2;
use crate::SliceCursor;

/// Update Player.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug, Default)]
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
    // update_velocity
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
    // used_return_potion
    pub hovering_down: bool,
    // }
    // bit flags {
    pub sleeping: bool,
    // }
    pub selected_item: u8,
    pub pos: Vec2,
    /// Velocity at the time of changing the movement.
    pub vel: Option<Vec2>,
    pub original_and_home_pos: Option<(Vec2, Vec2)>,
}

impl PacketBody for UpdatePlayer {
    const TAG: u8 = 13;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(
            &(0u8 // control
            | if self.key_up { 0x01 } else { 0 }
            | if self.key_down { 0x02 } else { 0 }
            | if self.key_left { 0x04 } else { 0 }
            | if self.key_right { 0x08 } else { 0 }
            | if self.key_jump { 0x10 } else { 0 }
            | if self.key_item_use { 0x20 } else { 0 }
            | if self.facing_right { 0x40 } else { 0 }),
        );
        cursor.write(
            &(0u8 // pulley
               | if self.pulley { 0x01 } else { 0 }
               | if self.pulley_right { 0x02 } else { 0 }
               | if self.vel.is_some() { 0x04 } else { 0 }
               | if self.vortex_stealth { 0x08 } else { 0 }
               | if self.gravity_flipped { 0x10 } else { 0 }
               | if self.shield_raised { 0x20 } else { 0 }),
        );
        cursor.write(
            &(0u8 // misc
                | if self.hovering_up { 0x01 } else { 0 }
                | if self.void_vault { 0x02 } else { 0 }
                | if self.sitting { 0x04 } else { 0 }
                | if self.downed_dd2 { 0x08 } else { 0 }
                | if self.petting_animal { 0x10 } else { 0 }
                | if self.petting_small_animal { 0x20 } else { 0 }
                | if self.original_and_home_pos.is_some() { 0x40 } else { 0 }
                | if self.hovering_down { 0x80 } else { 0 }),
        );
        cursor.write(
            &(0u8 // sleeping info
                | if self.sleeping { 0x01 } else { 0 }),
        );
        cursor.write(&self.selected_item);
        cursor.write(&self.pos);
        if let Some(vel) = self.vel.as_ref() {
            cursor.write(vel);
        }
        if let Some((original_pos, home_pos)) = self.original_and_home_pos.as_ref() {
            cursor.write(original_pos);
            cursor.write(home_pos);
        }
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let player_id = cursor.read();
        let control = cursor.read::<u8>();
        let pulley = cursor.read::<u8>();
        let misc = cursor.read::<u8>();
        let sleeping_info = cursor.read::<u8>();
        let selected_item = cursor.read();
        let pos = cursor.read();
        let vel = if pulley & 0x04 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let original_and_home_pos = if misc & 0x40 != 0 {
            Some((cursor.read(), cursor.read()))
        } else {
            None
        };
        Self {
            player_id,
            key_up: control & 0x01 != 0,
            key_down: control & 0x02 != 0,
            key_left: control & 0x04 != 0,
            key_right: control & 0x08 != 0,
            key_jump: control & 0x10 != 0,
            key_item_use: control & 0x20 != 0,
            facing_right: control & 0x40 != 0,
            pulley: pulley & 0x01 != 0,
            pulley_right: pulley & 0x02 != 0,
            vortex_stealth: pulley & 0x08 != 0,
            gravity_flipped: pulley & 0x10 != 0,
            shield_raised: pulley & 0x20 != 0,
            hovering_up: misc & 0x01 != 0,
            void_vault: misc & 0x02 != 0,
            sitting: misc & 0x04 != 0,
            downed_dd2: misc & 0x08 != 0,
            petting_animal: misc & 0x10 != 0,
            petting_small_animal: misc & 0x20 != 0,
            hovering_down: misc & 0x80 != 0,
            sleeping: sleeping_info & 0x01 != 0,
            selected_item,
            pos,
            vel,
            original_and_home_pos,
        }
    }
}
