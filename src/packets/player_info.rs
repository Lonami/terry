use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Player information.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerInfo {
    pub player_id: u8,
    pub skin_varient: u8,
    /// If > 162 then Set To 0
    pub hair: u8,
    pub name: String,
    pub hair_dye: u8,
    pub hide_visuals: u8,
    pub hide_visuals_2: u8,
    pub hide_misc: u8,
    pub hair_color: Color,
    pub skin_color: Color,
    pub eye_color: Color,
    pub shirt_color: Color,
    pub under_shirt_color: Color,
    pub pants_color: Color,
    pub shoe_color: Color,
    pub difficulty: u8,
}

impl PacketBody for PlayerInfo {
    const TAG: u8 = 4;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.skin_varient);
        cursor.write(&self.hair);
        cursor.write(&self.name);
        cursor.write(&self.hair_dye);
        cursor.write(&self.hide_visuals);
        cursor.write(&self.hide_visuals_2);
        cursor.write(&self.hide_misc);
        cursor.write(&self.hair_color);
        cursor.write(&self.skin_color);
        cursor.write(&self.eye_color);
        cursor.write(&self.shirt_color);
        cursor.write(&self.under_shirt_color);
        cursor.write(&self.pants_color);
        cursor.write(&self.shoe_color);
        cursor.write(&self.difficulty);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            skin_varient: cursor.read(),
            hair: cursor.read(),
            name: cursor.read(),
            hair_dye: cursor.read(),
            hide_visuals: cursor.read(),
            hide_visuals_2: cursor.read(),
            hide_misc: cursor.read(),
            hair_color: cursor.read(),
            skin_color: cursor.read(),
            eye_color: cursor.read(),
            shirt_color: cursor.read(),
            under_shirt_color: cursor.read(),
            pants_color: cursor.read(),
            shoe_color: cursor.read(),
            difficulty: cursor.read(),
        }
    }
}
