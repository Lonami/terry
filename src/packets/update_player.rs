use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Update Player.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct UpdatePlayer {
    pub player_id: u8,
    /// BitFlags: 1 = ControlUp, 2 = ControlDown, 4 = ControlLeft, 8 = ControlRight, 16 = ControlJump, 32 = ControlUseItem, 64 = Direction
    pub control: u8,
    /// BitFlags: 1 = Pulley Enabled, 2 = Direction, 4 = UpdateVelocity, 8 = VortexStealthActive, 16 = GravityDirection, 32 = ShieldRaised
    pub pulley: u8,
    /// BitFlags: 1 = HoveringUp, 2 = VoidVaultEnabled, 4 = Sitting, 8 = DownedDD2Event, 16 = IsPettingAnimal, 32 = IsPettingSmallAnimal, 64 = UsedPotionofReturn, 128 = HoveringDown
    pub misc: u8,
    /// BitFlags: 1 = IsSleeping
    pub sleepinginfo: u8,
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
