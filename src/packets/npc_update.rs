use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// NPC update, such as movement.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct NpcUpdate {
    pub npc_id: i16,
    pub position_x: i32, /* single */
    pub position_y: i32, /* single */
    pub velocity_x: i32, /* single */
    pub velocity_y: i32, /* single */
    /// Player ID
    pub target: u16,
    /// BitFlags: 1 = Direction, 2 = DirectionY, 4 = AI
    pub npcflags1: u8,
    /// BitFlags: 1 = StatsScaled, 2 = SpawnedFromStatue, 4 = StrengthMultiplier
    pub npcflags2: u8,
    /// Only sent for each true AI flag in NpcFlags1
    pub ai: i32, /* single */
    pub npc_netid: i16,
    /// Only sent if StatsScaled flag is true
    pub playercountformultiplayerdifficultyoverride: u8,
    /// Only sent if StrengthMultiplier flag is true
    pub strength_multiplier: i32, /* single */
    /// The size of Life (in bytes), only sent if LifeMax flag is not true
    pub lifebytes: u8,
    /// Byte, Int16, or Int32 according to LifeBytes, only sent if LifeMax flag is not true
    pub life: i32, /* variable */
    /// Only present if NPC is catchable
    pub releaseowner: u8,
}

impl PacketBody for NpcUpdate {
    const TAG: u8 = 23;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.npc_id);
        cursor.write(&self.position_x);
        cursor.write(&self.position_y);
        cursor.write(&self.velocity_x);
        cursor.write(&self.velocity_y);
        cursor.write(&self.target);
        cursor.write(&self.npcflags1);
        cursor.write(&self.npcflags2);
        cursor.write(&self.ai);
        cursor.write(&self.npc_netid);
        cursor.write(&self.playercountformultiplayerdifficultyoverride);
        cursor.write(&self.strength_multiplier);
        cursor.write(&self.lifebytes);
        cursor.write(&self.life); // TODO this field is variable
        cursor.write(&self.releaseowner);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            npc_id: cursor.read(),
            position_x: cursor.read(),
            position_y: cursor.read(),
            velocity_x: cursor.read(),
            velocity_y: cursor.read(),
            target: cursor.read(),
            npcflags1: cursor.read(),
            npcflags2: cursor.read(),
            ai: cursor.read(),
            npc_netid: cursor.read(),
            playercountformultiplayerdifficultyoverride: cursor.read(),
            strength_multiplier: cursor.read(),
            lifebytes: cursor.read(),
            life: cursor.read(), // TODO this field is variable
            releaseowner: cursor.read(),
        }
    }
}
