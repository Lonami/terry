use crate::packets::{PacketBody, RGB};
use crate::serialization::SliceCursor;

/// Player information, sent on login or when the player looks change.
pub struct PlayerInfo {
    pub skin_variant: u8,
    pub hair_variant: u8,
    pub name: String,
    pub hair_dye: u8,
    pub visible_accesories_flags: u16,
    pub hide_misc: bool,
    pub hair_color: RGB,
    pub skin_color: RGB,
    pub eye_color: RGB,
    pub shirt_color: RGB,
    pub undershirt_color: RGB,
    pub pants_color: RGB,
    pub shoes_color: RGB,
    pub difficulty_flags: u8,
}

impl PacketBody for PlayerInfo {
    const TAG: u8 = 4;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.skin_variant);
        cursor.write(&self.hair_variant);
        cursor.write(&self.name);
        cursor.write(&self.hair_dye);
        cursor.write(&self.visible_accesories_flags);
        cursor.write(&self.hide_misc);
        cursor.write(&self.hair_color);
        cursor.write(&self.skin_color);
        cursor.write(&self.eye_color);
        cursor.write(&self.shirt_color);
        cursor.write(&self.undershirt_color);
        cursor.write(&self.pants_color);
        cursor.write(&self.shoes_color);
        cursor.write(&self.difficulty_flags);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            skin_variant: cursor.read(),
            hair_variant: cursor.read(),
            name: cursor.read(),
            hair_dye: cursor.read(),
            visible_accesories_flags: cursor.read(),
            hide_misc: cursor.read(),
            hair_color: cursor.read(),
            skin_color: cursor.read(),
            eye_color: cursor.read(),
            shirt_color: cursor.read(),
            undershirt_color: cursor.read(),
            pants_color: cursor.read(),
            shoes_color: cursor.read(),
            difficulty_flags: cursor.read(),
        }
    }
}
