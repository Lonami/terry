use crate::serde::{packet_struct, serializable_bitflags};
use crate::structures::Rgb;

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
        pub hair_color: Rgb,
        pub skin_color: Rgb,
        pub eye_color: Rgb,
        pub shirt_color: Rgb,
        pub undershirt_color: Rgb,
        pub pants_color: Rgb,
        pub shoes_color: Rgb,
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
            hair_color: Rgb {
                r: 215,
                g: 90,
                b: 55,
            },
            skin_color: Rgb {
                r: 255,
                g: 125,
                b: 90,
            },
            eye_color: Rgb {
                r: 105,
                g: 90,
                b: 75,
            },
            shirt_color: Rgb {
                r: 175,
                g: 165,
                b: 140,
            },
            undershirt_color: Rgb {
                r: 160,
                g: 180,
                b: 215,
            },
            pants_color: Rgb {
                r: 255,
                g: 230,
                b: 175,
            },
            shoes_color: Rgb {
                r: 160,
                g: 105,
                b: 60,
            },
            difficulty: Difficulty::SOFTCORE,
            torches: Torches::empty(),
        }
    }
}
