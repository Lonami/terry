use crate::packets::{Packet, RGB};
use std::convert::TryInto;

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

impl Packet for PlayerInfo {
    const TAG: u8 = 4;

    fn append_body(&self, buf: &mut Vec<u8>) {
        buf.push(self.skin_variant);
        buf.push(self.hair_variant);
        buf.push(self.name.len().try_into().expect("name too long"));
        buf.extend(self.name.as_bytes());
        buf.push(self.hair_dye);
        buf.extend(&self.visible_accesories_flags.to_le_bytes());
        buf.push(self.hide_misc as u8);
        self.hair_color.append_body(buf);
        self.skin_color.append_body(buf);
        self.eye_color.append_body(buf);
        self.shirt_color.append_body(buf);
        self.undershirt_color.append_body(buf);
        self.pants_color.append_body(buf);
        self.shoes_color.append_body(buf);
        buf.push(self.difficulty_flags);
    }
}
