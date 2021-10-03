use crate::packets::PacketBody;
use crate::structures::{serializable_bitflags, Vec2};
use crate::SliceCursor;

serializable_bitflags! {
    pub struct KeyPress: u8 {
        const UP = 0x01;
        const DOWN = 0x02;
        const LEFT = 0x04;
        const RIGHT = 0x08;
        const JUMP = 0x10;
        const USE = 0x20;
        const FACING_RIGHT = 0x40;
    }
}

serializable_bitflags! {
    pub struct PulleyMode: u8 {
        const PULLEY = 0x01;
        const PYLLEY_RIGHT = 0x02;
        const HAS_VEL = 0x04;
        const VORTEX_STEALTH = 0x08;
        const GRAVITY_FLIPPED = 0x10;
        const SHIELD_RAISED = 0x20;
    }
}

serializable_bitflags! {
    pub struct PlayerAction: u8 {
        const HOVERING_UP = 0x01;
        const VOID_VAULT = 0x02;
        const SITTING = 0x04;
        const DOWNED_DD2 = 0x08;
        const PETTING_ANIMAL = 0x10;
        const PETTING_SMALL_ANIMAL = 0x20;
        const HAS_ORIG_AND_HOME_POS = 0x40;
        const HOVERING_DOWN = 0x80;
    }
}

serializable_bitflags! {
    pub struct SleepInfo: u8 {
        const SLEEPING = 0x01;
    }
}

/// Update Player.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug, Default)]
pub struct UpdatePlayer {
    pub player_id: u8,
    pub keys: KeyPress,
    pub pulley: PulleyMode,
    pub action: PlayerAction,
    pub sleep_info: SleepInfo,
    pub selected_item: u8,
    pub pos: Vec2,
    /// Velocity at the time of changing the movement
    pub vel: Option<Vec2>,
    pub original_and_home_pos: Option<(Vec2, Vec2)>,
}

impl PacketBody for UpdatePlayer {
    const TAG: u8 = 13;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.keys);
        cursor.write(&self.pulley);
        cursor.write(&self.action);
        cursor.write(&self.sleep_info);
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
        let keys = cursor.read();
        let pulley = cursor.read::<PulleyMode>();
        let action = cursor.read::<PlayerAction>();
        let sleep_info = cursor.read();
        let selected_item = cursor.read();
        let pos = cursor.read();

        let vel = pulley.contains(PulleyMode::HAS_VEL).then(|| cursor.read());

        let original_and_home_pos = action
            .contains(PlayerAction::HAS_ORIG_AND_HOME_POS)
            .then(|| (cursor.read(), cursor.read()));

        Self {
            player_id,
            keys,
            pulley,
            action,
            sleep_info,
            selected_item,
            pos,
            vel,
            original_and_home_pos,
        }
    }
}
