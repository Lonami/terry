use crate::packets::PacketBody;
use crate::structures::serializable_bitflags;
use crate::SliceCursor;

serializable_bitflags! {
    pub struct DayInfo: u8 {
        const DAY_TIME = 0x01;
        const BLOOD_MOON = 0x02;
        const ECLIPSE = 0x04;
    }
}

serializable_bitflags! {
    pub struct EventInfo: u64 {
        const SHADOW_ORB_SMASHED = 0x0000_0000_0000_0001;
        const DOWNED_BOSS_1 = 0x0000_0000_0000_0002;
        const DOWNED_BOSS_2 = 0x0000_0000_0000_0004;
        const DOWNED_BOSS_3 = 0x0000_0000_0000_0008;
        const HARD_MODE = 0x0000_0000_0000_0010;
        const DOWNED_CLOWN = 0x0000_0000_0000_0020;
        const SERVER_SIDE_CHARACTER = 0x0000_0000_0000_0040;
        const DOWNED_PLANT_BOSS = 0x0000_0000_0000_0080;
        const MECH_BOSS_DOWNED = 0x0000_0000_0000_0100;
        const MECH_BOSS_DOWNED_2 = 0x0000_0000_0000_0200;
        const MECH_BOSS_DOWNED_3 = 0x0000_0000_0000_0400;
        const MECH_BOSS_ANY_DOWNED = 0x0000_0000_0000_0800;
        const CLOUD_BG = 0x0000_0000_0000_1000;
        const CRIMSON = 0x0000_0000_0000_2000;
        const PUMPKIN_MOON = 0x0000_0000_0000_4000;
        const SNOW_MOON = 0x0000_0000_0000_8000;
        const EXPERT_MODE = 0x0000_0000_0001_0000;
        const FASTFORWARDTIME = 0x0000_0000_0002_0000;
        const SLIME_RAIN = 0x0000_0000_0004_0000;
        const DOWNED_SLIME_KING = 0x0000_0000_0008_0000;
        const DOWNED_QUEEN_BEE = 0x0000_0000_0010_0000;
        const DOWNED_FISHRON = 0x0000_0000_0020_0000;
        const DOWNED_MARTIANS = 0x0000_0000_0040_0000;
        const DOWNED_ANCIENT_CULTIST = 0x0000_0000_0080_0000;
        const DOWNED_MOON_LORD = 0x0000_0000_0100_0000;
        const DOWNED_PUMKING = 0x0000_0000_0200_0000;
        const DOWNED_MOURNING_WOOD = 0x0000_0000_0400_0000;
        const DOWNED_ICE_QUEEN = 0x0000_0000_0800_0000;
        const DOWNED_SANTANK = 0x0000_0000_1000_0000;
        const DOWNED_EVERSCREAM = 0x0000_0000_2000_0000;
        const DOWNED_GOLEM = 0x0000_0000_4000_0000;
        const BIRTHDAY_PARTY = 0x0000_0000_8000_0000;
        const DOWNED_PIRATES = 0x0000_0001_0000_0000;
        const DOWNED_FROST_LEGION = 0x0000_0002_0000_0000;
        const DOWNED_GOBLINS = 0x0000_0004_0000_0000;
        const SANDSTORM = 0x0000_0008_0000_0000;
        const DD2_EVENT = 0x0000_0010_0000_0000;
        const DOWNED_DD2_TIER_1 = 0x0000_0020_0000_0000;
        const DOWNED_DD2_TIER_2 = 0x0000_0040_0000_0000;
        const DOWNED_DD2_TIER_3 = 0x0000_0080_0000_0000;
        const COMBAT_BOOK_USED = 0x0000_0100_0000_0000;
        const MANUAL_LANTERNS = 0x0000_0200_0000_0000;
        const DOWNED_SOLAR_TOWER = 0x0000_0400_0000_0000;
        const DOWNED_VORTEX_TOWER = 0x0000_0800_0000_0000;
        const DOWNED_TOWER_NEBULA = 0x0000_1000_0000_0000;
        const DOWNED_STARDUST_TOWER = 0x0000_2000_0000_0000;
        const FORCE_HALLOWEEN_DAY = 0x0000_4000_0000_0000;
        const FORCE_XMAS_DAY = 0x0000_8000_0000_0000;
        const BOUGHT_CAT = 0x0001_0000_0000_0000;
        const BOUGHT_DOG = 0x0002_0000_0000_0000;
        const BOUGHT_BUNNY = 0x0004_0000_0000_0000;
        const FREE_CAKE = 0x0008_0000_0000_0000;
        const DRUNK_WORLD = 0x0010_0000_0000_0000;
        const DOWNED_EMPRESS_OF_LIGHT = 0x0020_0000_0000_0000;
        const DOWNED_QUEEN_SLIME = 0x0040_0000_0000_0000;
        const GET_GOOD_WORLD = 0x0080_0000_0000_0000;
    }
}

/// World Info.
///
/// Direction: Server -> Client.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct WorldInfo {
    pub time: i32,
    pub day_info: DayInfo,
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
    pub event_info: EventInfo,
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

impl Eq for WorldInfo {}

impl PacketBody for WorldInfo {
    const TAG: u8 = 7;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.time);
        cursor.write(&self.day_info);
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
        cursor.write(&self.event_info);
        self.ore_tiers_tiles.iter().for_each(|t| cursor.write(t));
        cursor.write(&self.invasion_type);
        cursor.write(&self.lobby_id);
        cursor.write(&self.sandstorm_severity);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        let time = cursor.read();
        let day_info = cursor.read();
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
        let event_info = cursor.read();
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

        Self {
            time,
            day_info,
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
            event_info,
            ore_tiers_tiles,
            invasion_type,
            lobby_id,
            sandstorm_severity,
        }
    }
}
