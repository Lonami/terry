use crate::packets::packet_struct;
use crate::structures::{serializable_bitflags, RGB};

serializable_bitflags! {
    pub struct Difficulty: u8 {
        const SOFTCORE = 0x00;
        const MEDIUMCORE = 0x01;
        const HARDCORE = 0x02;
        const EXTRA_ACCESSORY = 0x04;
        const CREATIVE = 0x08;
    }
}

serializable_bitflags! {
    pub struct Torches: u8 {
        const USING_BIOME = 0x01;
        const HAPPY_FUN = 0x02;
        const UNLOCKED_BIOME = 0x04;
    }
}

packet_struct! {
    /// Player information.
    ///
    /// Direction: Server <-> Client (Sync).
    pub struct PlayerInfo {
        const TAG = 4;

        pub player_id: u8,
        pub skin_variant: u8,
        /// Must be 0 if it's set to something greater than 162
        pub hair_variant: u8,
        pub name: String,
        pub hair_dye: u8,
        pub hide_visuals: u16,
        pub hide_misc: u8,
        pub hair_color: RGB,
        pub skin_color: RGB,
        pub eye_color: RGB,
        pub shirt_color: RGB,
        pub undershirt_color: RGB,
        pub pants_color: RGB,
        pub shoes_color: RGB,
        pub difficulty: Difficulty,
        pub torches: Torches,
    }
}

impl PlayerInfo {
    pub fn terry() -> Self {
        Self {
            player_id: 0,
            skin_variant: 0,
            hair_variant: 0,
            name: "terry".to_string(),
            hair_dye: 0,
            hide_visuals: 0,
            hide_misc: 0,
            hair_color: RGB {
                r: 215,
                g: 90,
                b: 55,
            },
            skin_color: RGB {
                r: 255,
                g: 125,
                b: 90,
            },
            eye_color: RGB {
                r: 105,
                g: 90,
                b: 75,
            },
            shirt_color: RGB {
                r: 175,
                g: 165,
                b: 140,
            },
            undershirt_color: RGB {
                r: 160,
                g: 180,
                b: 215,
            },
            pants_color: RGB {
                r: 255,
                g: 230,
                b: 175,
            },
            shoes_color: RGB {
                r: 160,
                g: 105,
                b: 60,
            },
            difficulty: Difficulty::SOFTCORE,
            torches: Torches::empty(),
        }
    }
}
