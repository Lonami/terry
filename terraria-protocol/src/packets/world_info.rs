use crate::packets::PacketBody;
use crate::SliceCursor;

/// World Info.
///
/// Direction: Server -> Client.
#[derive(Debug, Clone, Default)]
pub struct WorldInfo {
    pub time: i32,
    // bitflags {
    pub day_time: bool,
    pub blood_moon: bool,
    pub eclipse: bool,
    // }
    pub moon_phase: u8,
    pub max_tiles_x: i16,
    pub max_tiles_y: i16,
    pub spawn_x: i16,
    pub spawn_y: i16,
    pub world_surface: i16,
    pub rock_layer: i16,
    pub world_id: i32,
    pub world_name: String,
    pub game_mode: u8,
    pub world_unique_id: [u8; 16],
    /// Major and minor version
    pub world_generator_version: [i32; 2],
    pub moon_type: u8,
    pub tree_background: u8,
    pub corruption_background: u8,
    pub jungle_background: u8,
    pub snow_background: u8,
    pub hallow_background: u8,
    pub crimson_background: u8,
    pub desert_background: u8,
    pub ocean_background: u8,
    pub unknown_background: [u8; 5],
    pub ice_back_style: u8,
    pub jungle_back_style: u8,
    pub hell_back_style: u8,
    pub wind_speed_set: f32,
    pub cloud_number: u8,
    pub trees: [i32; 3],
    pub tree_styles: [u8; 4],
    pub cave_backs: [i32; 3],
    pub cave_back_styles: [u8; 4],
    pub forest_tree_top_styles: [u8; 4],
    pub corruption_tree_top_style: u8,
    pub jungle_tree_top_style: u8,
    pub snow_tree_top_style: u8,
    pub hallow_tree_top_style: u8,
    pub crimson_tree_top_style: u8,
    pub desert_tree_top_style: u8,
    pub ocean_tree_top_style: u8,
    pub glowing_mushroom_tree_top_style: u8,
    pub underworld_tree_top_style: u8,
    pub rain: f32,
    // bitflags {
    pub shadow_orb_smashed: bool,
    pub downed_boss_1: bool,
    pub downed_boss_2: bool,
    pub downed_boss_3: bool,
    pub hard_mode: bool,
    pub downed_clown: bool,
    pub server_side_character: bool,
    pub downed_plant_boss: bool,
    pub mech_boss_downed: bool,
    pub mech_boss_downed_2: bool,
    pub mech_boss_downed_3: bool,
    pub mech_boss_any_downed: bool,
    pub cloud_bg: bool,
    pub crimson: bool,
    pub pumpkin_moon: bool,
    pub snow_moon: bool,
    pub expert_mode: bool,
    pub fastforwardtime: bool,
    pub slime_rain: bool,
    pub downed_slime_king: bool,
    pub downed_queen_bee: bool,
    pub downed_fishron: bool,
    pub downed_martians: bool,
    pub downed_ancient_cultist: bool,
    pub downed_moon_lord: bool,
    pub downed_pumking: bool,
    pub downed_mourning_wood: bool,
    pub downed_ice_queen: bool,
    pub downed_santank: bool,
    pub downed_everscream: bool,
    pub downed_golem: bool,
    pub birthday_party: bool,
    pub downed_pirates: bool,
    pub downed_frost_legion: bool,
    pub downed_goblins: bool,
    pub sandstorm: bool,
    pub dd2_event: bool,
    pub downed_dd2_tier_1: bool,
    pub downed_dd2_tier_2: bool,
    pub downed_dd2_tier_3: bool,
    pub combat_book_used: bool,
    pub manual_lanterns: bool,
    pub downed_solar_tower: bool,
    pub downed_vortex_tower: bool,
    pub downed_tower_nebula: bool,
    pub downed_stardust_tower: bool,
    pub force_halloween_day: bool,
    pub force_xmas_day: bool,
    pub bought_cat: bool,
    pub bought_dog: bool,
    pub bought_bunny: bool,
    pub free_cake: bool,
    pub drunk_world: bool,
    pub downed_empress_of_light: bool,
    pub downed_queen_slime: bool,
    // }
    /// The respective tier indices, names and possible tile IDs are:
    ///
    /// * Tier 0: Copper (tile ID 7 or 166)
    /// * Tier 1: Iron (tile ID 6 or 167)
    /// * Tier 2: Silver (tile ID 9 or 168)
    /// * Tier 3: Gold (tile ID 8 or 169)
    /// * Tier 4: Cobalt (tile ID 107 or 221, -1 if not decided)
    /// * Tier 5: Mythril (tile ID 108 or 222, -1 if not decided)
    /// * Tier 6: Adamantite (tile ID 111 or 223, -1 if not decided)
    pub ore_tiers_tiles: [i16; 7],
    pub invasion_type: i8,
    pub lobby_id: u64,
    pub sandstorm_severity: f32,
}

impl PacketBody for WorldInfo {
    const TAG: u8 = 7;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.time);
        cursor.write(
            &(0 // day and moon info
            | if self.day_time { 0x01 } else { 0 }
            | if self.blood_moon { 0x02 } else { 0 }
            | if self.eclipse { 0x04 } else { 0 }),
        );
        cursor.write(&self.moon_phase);
        cursor.write(&self.max_tiles_x);
        cursor.write(&self.max_tiles_y);
        cursor.write(&self.spawn_x);
        cursor.write(&self.spawn_y);
        cursor.write(&self.world_surface);
        cursor.write(&self.rock_layer);
        cursor.write(&self.world_id);
        cursor.write(&self.world_name);
        cursor.write(&self.game_mode);
        self.world_unique_id.iter().for_each(|i| cursor.write(i));
        self.world_generator_version
            .iter()
            .for_each(|x| cursor.write(x));
        cursor.write(&self.moon_type);
        cursor.write(&self.tree_background);
        cursor.write(&self.corruption_background);
        cursor.write(&self.jungle_background);
        cursor.write(&self.snow_background);
        cursor.write(&self.hallow_background);
        cursor.write(&self.crimson_background);
        cursor.write(&self.desert_background);
        cursor.write(&self.ocean_background);
        self.unknown_background.iter().for_each(|x| cursor.write(x));
        cursor.write(&self.ice_back_style);
        cursor.write(&self.jungle_back_style);
        cursor.write(&self.hell_back_style);
        cursor.write(&self.wind_speed_set);
        cursor.write(&self.cloud_number);
        self.trees.iter().for_each(|x| cursor.write(x));
        self.tree_styles.iter().for_each(|x| cursor.write(x));
        self.cave_backs.iter().for_each(|x| cursor.write(x));
        self.cave_back_styles.iter().for_each(|x| cursor.write(x));
        self.forest_tree_top_styles
            .iter()
            .for_each(|x| cursor.write(x));
        cursor.write(&self.corruption_tree_top_style);
        cursor.write(&self.jungle_tree_top_style);
        cursor.write(&self.snow_tree_top_style);
        cursor.write(&self.hallow_tree_top_style);
        cursor.write(&self.crimson_tree_top_style);
        cursor.write(&self.desert_tree_top_style);
        cursor.write(&self.ocean_tree_top_style);
        cursor.write(&self.glowing_mushroom_tree_top_style);
        cursor.write(&self.underworld_tree_top_style);
        cursor.write(&self.rain);
        cursor.write(
            &(0 // event info[0]
            | if self.shadow_orb_smashed { 0x01 } else { 0 }
            | if self.downed_boss_1 { 0x02 } else { 0 }
            | if self.downed_boss_2 { 0x04 } else { 0 }
            | if self.downed_boss_3 { 0x08 } else { 0 }
            | if self.hard_mode { 0x10 } else { 0 }
            | if self.downed_clown { 0x20 } else { 0 }
            | if self.server_side_character { 0x40 } else { 0 }
            | if self.downed_plant_boss { 0x80 } else { 0 }),
        );
        cursor.write(
            &(0 // event info[1]
            | if self.mech_boss_downed { 0x01 } else { 0 }
            | if self.mech_boss_downed_2 { 0x02 } else { 0 }
            | if self.mech_boss_downed_3 { 0x04 } else { 0 }
            | if self.mech_boss_any_downed { 0x08 } else { 0 }
            | if self.cloud_bg { 0x10 } else { 0 }
            | if self.crimson { 0x20 } else { 0 }
            | if self.pumpkin_moon { 0x40 } else { 0 }
            | if self.snow_moon { 0x80 } else { 0 }),
        );
        cursor.write(
            &(0 // event info[2]
            | if self.expert_mode { 0x01 } else { 0 }
            | if self.fastforwardtime { 0x02 } else { 0 }
            | if self.slime_rain { 0x04 } else { 0 }
            | if self.downed_slime_king { 0x08 } else { 0 }
            | if self.downed_queen_bee { 0x10 } else { 0 }
            | if self.downed_fishron { 0x20 } else { 0 }
            | if self.downed_martians { 0x40 } else { 0 }
            | if self.downed_ancient_cultist { 0x80 } else { 0 }),
        );
        cursor.write(
            &(0 // event info[3]
            | if self.downed_moon_lord { 0x01 } else { 0 }
            | if self.downed_pumking { 0x02 } else { 0 }
            | if self.downed_mourning_wood { 0x04 } else { 0 }
            | if self.downed_ice_queen { 0x08 } else { 0 }
            | if self.downed_santank { 0x10 } else { 0 }
            | if self.downed_everscream { 0x20 } else { 0 }
            | if self.downed_golem { 0x40 } else { 0 }
            | if self.birthday_party { 0x80 } else { 0 }),
        );
        cursor.write(
            &(0 // event info[4]
            | if self.downed_pirates { 0x01 } else { 0 }
            | if self.downed_frost_legion { 0x02 } else { 0 }
            | if self.downed_goblins { 0x04 } else { 0 }
            | if self.sandstorm { 0x08 } else { 0 }
            | if self.dd2_event { 0x10 } else { 0 }
            | if self.downed_dd2_tier_1 { 0x20 } else { 0 }
            | if self.downed_dd2_tier_2 { 0x40 } else { 0 }
            | if self.downed_dd2_tier_3 { 0x80 } else { 0 }),
        );
        cursor.write(
            &(0 // event info[5]
            | if self.combat_book_used { 0x01 } else { 0 }
            | if self.manual_lanterns { 0x02 } else { 0 }
            | if self.downed_solar_tower { 0x04 } else { 0 }
            | if self.downed_vortex_tower { 0x08 } else { 0 }
            | if self.downed_tower_nebula { 0x10 } else { 0 }
            | if self.downed_stardust_tower { 0x20 } else { 0 }
            | if self.force_halloween_day { 0x40 } else { 0 }
            | if self.force_xmas_day { 0x80 } else { 0 }),
        );
        cursor.write(
            &(0 // event info[6]
            | if self.bought_cat { 0x01 } else { 0 }
            | if self.bought_dog { 0x02 } else { 0 }
            | if self.bought_bunny { 0x04 } else { 0 }
            | if self.free_cake { 0x08 } else { 0 }
            | if self.drunk_world { 0x10 } else { 0 }
            | if self.downed_empress_of_light { 0x20 } else { 0 }
            | if self.downed_queen_slime { 0x40 } else { 0 }),
        );
        self.ore_tiers_tiles.iter().for_each(|t| cursor.write(t));
        cursor.write(&self.invasion_type);
        cursor.write(&self.lobby_id);
        cursor.write(&self.sandstorm_severity);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let time = cursor.read();
        let day_and_moon_info = cursor.read::<u8>();
        let moon_phase = cursor.read();
        let max_tiles_x = cursor.read();
        let max_tiles_y = cursor.read();
        let spawn_x = cursor.read();
        let spawn_y = cursor.read();
        let world_surface = cursor.read();
        let rock_layer = cursor.read();
        let world_id = cursor.read();
        let world_name = cursor.read();
        let game_mode = cursor.read();
        let mut world_unique_id = [0; 16];
        world_unique_id.iter_mut().for_each(|i| *i = cursor.read());
        let world_generator_version = [cursor.read(), cursor.read()];
        let moon_type = cursor.read();
        let tree_background = cursor.read();
        let corruption_background = cursor.read();
        let jungle_background = cursor.read();
        let snow_background = cursor.read();
        let hallow_background = cursor.read();
        let crimson_background = cursor.read();
        let desert_background = cursor.read();
        let ocean_background = cursor.read();
        let unknown_background = [
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
        ];
        let ice_back_style = cursor.read();
        let jungle_back_style = cursor.read();
        let hell_back_style = cursor.read();
        let wind_speed_set = cursor.read();
        let cloud_number = cursor.read();
        let trees = [cursor.read(), cursor.read(), cursor.read()];
        let tree_styles = [cursor.read(), cursor.read(), cursor.read(), cursor.read()];
        let cave_backs = [cursor.read(), cursor.read(), cursor.read()];
        let cave_back_styles = [cursor.read(), cursor.read(), cursor.read(), cursor.read()];
        let forest_tree_top_styles = [cursor.read(), cursor.read(), cursor.read(), cursor.read()];
        let corruption_tree_top_style = cursor.read();
        let jungle_tree_top_style = cursor.read();
        let snow_tree_top_style = cursor.read();
        let hallow_tree_top_style = cursor.read();
        let crimson_tree_top_style = cursor.read();
        let desert_tree_top_style = cursor.read();
        let ocean_tree_top_style = cursor.read();
        let glowing_mushroom_tree_top_style = cursor.read();
        let underworld_tree_top_style = cursor.read();
        let rain = cursor.read();
        let event_info: [u8; 7] = [
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
        ];
        let ore_tiers_tiles = [
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
            cursor.read(),
        ];
        let invasion_type = cursor.read();
        let lobby_id = cursor.read();
        let sandstorm_severity = cursor.read();

        dbg!(Self {
            time,
            day_time: day_and_moon_info & 0x01 != 0,
            blood_moon: day_and_moon_info & 0x02 != 0,
            eclipse: day_and_moon_info & 0x04 != 0,
            moon_phase,
            max_tiles_x,
            max_tiles_y,
            spawn_x,
            spawn_y,
            world_surface,
            rock_layer,
            world_id,
            world_name,
            game_mode,
            world_unique_id,
            world_generator_version,
            moon_type,
            tree_background,
            corruption_background,
            jungle_background,
            snow_background,
            hallow_background,
            crimson_background,
            desert_background,
            ocean_background,
            unknown_background,
            ice_back_style,
            jungle_back_style,
            hell_back_style,
            wind_speed_set,
            cloud_number,
            trees,
            tree_styles,
            cave_backs,
            cave_back_styles,
            forest_tree_top_styles,
            corruption_tree_top_style,
            jungle_tree_top_style,
            snow_tree_top_style,
            hallow_tree_top_style,
            crimson_tree_top_style,
            desert_tree_top_style,
            ocean_tree_top_style,
            glowing_mushroom_tree_top_style,
            underworld_tree_top_style,
            rain,
            shadow_orb_smashed: event_info[0] & 0x01 != 0,
            downed_boss_1: event_info[0] & 0x02 != 0,
            downed_boss_2: event_info[0] & 0x04 != 0,
            downed_boss_3: event_info[0] & 0x08 != 0,
            hard_mode: event_info[0] & 0x10 != 0,
            downed_clown: event_info[0] & 0x20 != 0,
            server_side_character: event_info[0] & 0x40 != 0,
            downed_plant_boss: event_info[0] & 0x80 != 0,
            mech_boss_downed: event_info[1] & 0x01 != 0,
            mech_boss_downed_2: event_info[1] & 0x02 != 0,
            mech_boss_downed_3: event_info[1] & 0x04 != 0,
            mech_boss_any_downed: event_info[1] & 0x08 != 0,
            cloud_bg: event_info[1] & 0x10 != 0,
            crimson: event_info[1] & 0x20 != 0,
            pumpkin_moon: event_info[1] & 0x40 != 0,
            snow_moon: event_info[1] & 0x80 != 0,
            expert_mode: event_info[2] & 0x01 != 0,
            fastforwardtime: event_info[2] & 0x02 != 0,
            slime_rain: event_info[2] & 0x04 != 0,
            downed_slime_king: event_info[2] & 0x08 != 0,
            downed_queen_bee: event_info[2] & 0x10 != 0,
            downed_fishron: event_info[2] & 0x20 != 0,
            downed_martians: event_info[2] & 0x40 != 0,
            downed_ancient_cultist: event_info[2] & 0x80 != 0,
            downed_moon_lord: event_info[3] & 0x01 != 0,
            downed_pumking: event_info[3] & 0x02 != 0,
            downed_mourning_wood: event_info[3] & 0x04 != 0,
            downed_ice_queen: event_info[3] & 0x08 != 0,
            downed_santank: event_info[3] & 0x10 != 0,
            downed_everscream: event_info[3] & 0x20 != 0,
            downed_golem: event_info[3] & 0x40 != 0,
            birthday_party: event_info[3] & 0x80 != 0,
            downed_pirates: event_info[4] & 0x01 != 0,
            downed_frost_legion: event_info[4] & 0x02 != 0,
            downed_goblins: event_info[4] & 0x04 != 0,
            sandstorm: event_info[4] & 0x08 != 0,
            dd2_event: event_info[4] & 0x10 != 0,
            downed_dd2_tier_1: event_info[4] & 0x20 != 0,
            downed_dd2_tier_2: event_info[4] & 0x40 != 0,
            downed_dd2_tier_3: event_info[4] & 0x80 != 0,
            combat_book_used: event_info[5] & 0x01 != 0,
            manual_lanterns: event_info[5] & 0x02 != 0,
            downed_solar_tower: event_info[5] & 0x04 != 0,
            downed_vortex_tower: event_info[5] & 0x08 != 0,
            downed_tower_nebula: event_info[5] & 0x10 != 0,
            downed_stardust_tower: event_info[5] & 0x20 != 0,
            force_halloween_day: event_info[5] & 0x40 != 0,
            force_xmas_day: event_info[5] & 0x80 != 0,
            bought_cat: event_info[6] & 0x01 != 0,
            bought_dog: event_info[6] & 0x02 != 0,
            bought_bunny: event_info[6] & 0x04 != 0,
            free_cake: event_info[6] & 0x08 != 0,
            drunk_world: event_info[6] & 0x10 != 0,
            downed_empress_of_light: event_info[6] & 0x20 != 0,
            downed_queen_slime: event_info[6] & 0x40 != 0,
            ore_tiers_tiles,
            invasion_type,
            lobby_id,
            sandstorm_severity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_new_small_world_works() {
        let mut data = b"A.\x00\x00\x00\x02h\x10\xb0\x040\x08!\x01P\x01\xc8\x01$\xef\xc3\x17\x08botworld\x00\xff\xbf\x0fQ'\xde\x82@\xa1\xa3N\x03\xf2\xeba\x05\x01\x00\x00\x00\xe3\x00\x00\x00\x013\t\x07\x03\x02\x05\x06\x01\x01\x03\x01\x03\x01\x00\x00\x026^:\xbfe\xc3\x05\x00\x00h\x10\x00\x00h\x10\x00\x00\x04\x03\x00\x00<\x0b\x00\x00h\x10\x00\x00h\x10\x00\x00\x00\x07\x03\x07\x04\x03\x00\x00\x02\x05\x06\x01\x01\x03\x01\x03\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07\x00\x06\x00\xa8\x00\xa9\x00\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x00\x00\x00\xba\xc3\x8d>".to_vec();
        let mut cursor = SliceCursor::new(data.as_mut_slice());
        let world = WorldInfo::from_body(&mut cursor);
        assert_eq!(world.sandstorm_severity, 0.27688390016555786);
    }
}
