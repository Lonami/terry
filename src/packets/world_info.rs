use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// World Info.
///
/// Direction: Server -> Client.
#[derive(Debug)]
pub struct WorldInfo {
    pub time: i32,
    /// BitFlags: 1 = Day Time, 2 = Blood Moon, 4 = Eclipse
    pub day_and_moon_info: u8,
    pub moon_phase: u8,
    pub max_tiles_x: i16,
    pub max_tiles_y: i16,
    pub spawn_x: i16,
    pub spawn_y: i16,
    pub worldsurface: i16,
    pub rocklayer: i16,
    pub world_id: i32,
    pub world_name: String,
    pub game_mode: u8,
    pub world_unique_id: u8,
    pub world_generator_version: u64,
    pub moon_type: u8,
    pub tree_background: u8,
    pub corruption_background: u8,
    pub jungle_background: u8,
    pub snow_background: u8,
    pub hallow_background: u8,
    pub crimson_background: u8,
    pub desert_background: u8,
    pub ocean_background: u8,
    pub a_background: u8,
    pub b_background: u8,
    pub c_background: u8,
    pub d_background: u8,
    pub e_background: u8,
    pub ice_back_style: u8,
    pub jungle_back_style: u8,
    pub hell_back_style: u8,
    pub wind_speed_set: i32 /* single */ ,
    pub cloud_number: u8,
    pub tree_1: i32,
    pub tree_2: i32,
    pub tree_3: i32,
    pub tree_style_1: u8,
    pub tree_style_2: u8,
    pub tree_style_3: u8,
    pub tree_style_4: u8,
    pub cave_back_1: i32,
    pub cave_back_2: i32,
    pub cave_back_3: i32,
    pub cave_back_style_1: u8,
    pub cave_back_style_2: u8,
    pub cave_back_style_3: u8,
    pub cave_back_style_4: u8,
    pub forest_1_tree_top_style: i32,
    pub forest_2_tree_top_style: i32,
    pub forest_3_tree_top_style: i32,
    pub forest_4_tree_top_style: i32,
    pub corruption_tree_top_style: i32,
    pub jungle_tree_top_style: i32,
    pub snow_tree_top_style: i32,
    pub hallow_tree_top_style: i32,
    pub crimson_tree_top_style: i32,
    pub desert_tree_top_style: i32,
    pub ocean_tree_top_style: i32,
    pub glowing_mushroom_tree_top_style: i32,
    pub underworld_tree_top_style: i32,
    pub rain: i32 /* single */ ,
    /// BitFlags: 1 = Shadow Orb Smashed, 2 = Downed Boss 1, 4 = Downed Boss 2, 8 = Downed Boss 3, 16 = Hard Mode, 32 = Downed Clown, 64 = Server Side Character, 128 = Downed Plant Boss
    pub event_info: u8,
    /// BitFlags: 1 = Mech Boss Downed, 2 = Mech Boss Downed 2, 4 = Mech Boss Downed 3, 8 = Mech Boss Any Downed, 16 = Cloud BG, 32 = Crimson, 64 = Pumpkin Moon, 128 = Snow Moon
    pub event_info_2: u8,
    /// BitFlags: 1 = Expert Mode, 2 = FastForwardTime, 4 = Slime Rain, 8 = Downed Slime King, 16 = Downed Queen Bee, 32 = Downed Fishron, 64 = Downed Martians, 128 = Downed Ancient Cultist
    pub event_info_3: u8,
    /// BitFlags: 1 = Downed Moon Lord, 2 = Downed Pumking, 4 = Downed Mourning Wood, 8 = Downed Ice Queen, 16 = Downed Santank, 32 = Downed Everscream, 64 = Downed Golem, 128 = Birthday Party
    pub event_info_4: u8,
    /// BitFlags: 1 = Downed Pirates, 2 = Downed Frost Legion, 4 = Downed Goblins, 8 = Sandstorm, 16 = DD2 Event, 32 = Downed DD2 Tier 1, 64 = Downed DD2 Tier 2, 128 = Downed DD2 Tier 3
    pub event_info_5: u8,
    /// BitFlags: 1 = Combat Book Used, 2 = Manual Lanterns, 4 = Downed Solar Tower, 8 = Downed Vortex Tower, 16 = Downed Tower Nebula, 32 = Downed Stardust Tower, 64 = Force Halloween (day), 128 = Force XMas (day)
    pub event_info_6: u8,
    /// BitFlags: 1 = Bought Cat, 2 = Bought Dog, 4 = Bought Bunny, 8 = Free Cake, 16 = Drunk World, 32 = Downed Empress of Light, 64 = Downed Queen Slime
    pub event_info_7: u8,
    /// Tile ID 7 or 166
    pub copper_ore_tier: i16,
    /// Tile ID 6 or 167
    pub iron_ore_tier: i16,
    /// Tile ID 9 or 168
    pub silver_ore_tier: i16,
    /// Tile ID 8 or 169
    pub gold_ore_tier: i16,
    /// Tile ID 107 or 221
    pub cobalt_ore_tier: i16,
    /// Tile ID 108 or 222
    pub mythril_ore_tier: i16,
    /// Tile ID 111 or 223
    pub adamantite_ore_tier: i16,
    pub invasion_type: i8,
    pub lobby_id: u64,
    pub sandstorm_severity: i32 /* single */ ,
}

impl PacketBody for WorldInfo {
    const TAG: u8 = 7;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.time);
        cursor.write(&self.day_and_moon_info);
        cursor.write(&self.moon_phase);
        cursor.write(&self.max_tiles_x);
        cursor.write(&self.max_tiles_y);
        cursor.write(&self.spawn_x);
        cursor.write(&self.spawn_y);
        cursor.write(&self.worldsurface);
        cursor.write(&self.rocklayer);
        cursor.write(&self.world_id);
        cursor.write(&self.world_name);
        cursor.write(&self.game_mode);
        cursor.write(&self.world_unique_id);
        cursor.write(&self.world_generator_version);
        cursor.write(&self.moon_type);
        cursor.write(&self.tree_background);
        cursor.write(&self.corruption_background);
        cursor.write(&self.jungle_background);
        cursor.write(&self.snow_background);
        cursor.write(&self.hallow_background);
        cursor.write(&self.crimson_background);
        cursor.write(&self.desert_background);
        cursor.write(&self.ocean_background);
        cursor.write(&self.a_background);
        cursor.write(&self.b_background);
        cursor.write(&self.c_background);
        cursor.write(&self.d_background);
        cursor.write(&self.e_background);
        cursor.write(&self.ice_back_style);
        cursor.write(&self.jungle_back_style);
        cursor.write(&self.hell_back_style);
        cursor.write(&self.wind_speed_set);
        cursor.write(&self.cloud_number);
        cursor.write(&self.tree_1);
        cursor.write(&self.tree_2);
        cursor.write(&self.tree_3);
        cursor.write(&self.tree_style_1);
        cursor.write(&self.tree_style_2);
        cursor.write(&self.tree_style_3);
        cursor.write(&self.tree_style_4);
        cursor.write(&self.cave_back_1);
        cursor.write(&self.cave_back_2);
        cursor.write(&self.cave_back_3);
        cursor.write(&self.cave_back_style_1);
        cursor.write(&self.cave_back_style_2);
        cursor.write(&self.cave_back_style_3);
        cursor.write(&self.cave_back_style_4);
        cursor.write(&self.forest_1_tree_top_style);
        cursor.write(&self.forest_2_tree_top_style);
        cursor.write(&self.forest_3_tree_top_style);
        cursor.write(&self.forest_4_tree_top_style);
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
        cursor.write(&self.event_info_2);
        cursor.write(&self.event_info_3);
        cursor.write(&self.event_info_4);
        cursor.write(&self.event_info_5);
        cursor.write(&self.event_info_6);
        cursor.write(&self.event_info_7);
        cursor.write(&self.copper_ore_tier);
        cursor.write(&self.iron_ore_tier);
        cursor.write(&self.silver_ore_tier);
        cursor.write(&self.gold_ore_tier);
        cursor.write(&self.cobalt_ore_tier);
        cursor.write(&self.mythril_ore_tier);
        cursor.write(&self.adamantite_ore_tier);
        cursor.write(&self.invasion_type);
        cursor.write(&self.lobby_id);
        cursor.write(&self.sandstorm_severity);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            time: cursor.read(),
            day_and_moon_info: cursor.read(),
            moon_phase: cursor.read(),
            max_tiles_x: cursor.read(),
            max_tiles_y: cursor.read(),
            spawn_x: cursor.read(),
            spawn_y: cursor.read(),
            worldsurface: cursor.read(),
            rocklayer: cursor.read(),
            world_id: cursor.read(),
            world_name: cursor.read(),
            game_mode: cursor.read(),
            world_unique_id: cursor.read(),
            world_generator_version: cursor.read(),
            moon_type: cursor.read(),
            tree_background: cursor.read(),
            corruption_background: cursor.read(),
            jungle_background: cursor.read(),
            snow_background: cursor.read(),
            hallow_background: cursor.read(),
            crimson_background: cursor.read(),
            desert_background: cursor.read(),
            ocean_background: cursor.read(),
            a_background: cursor.read(),
            b_background: cursor.read(),
            c_background: cursor.read(),
            d_background: cursor.read(),
            e_background: cursor.read(),
            ice_back_style: cursor.read(),
            jungle_back_style: cursor.read(),
            hell_back_style: cursor.read(),
            wind_speed_set: cursor.read(),
            cloud_number: cursor.read(),
            tree_1: cursor.read(),
            tree_2: cursor.read(),
            tree_3: cursor.read(),
            tree_style_1: cursor.read(),
            tree_style_2: cursor.read(),
            tree_style_3: cursor.read(),
            tree_style_4: cursor.read(),
            cave_back_1: cursor.read(),
            cave_back_2: cursor.read(),
            cave_back_3: cursor.read(),
            cave_back_style_1: cursor.read(),
            cave_back_style_2: cursor.read(),
            cave_back_style_3: cursor.read(),
            cave_back_style_4: cursor.read(),
            forest_1_tree_top_style: cursor.read(),
            forest_2_tree_top_style: cursor.read(),
            forest_3_tree_top_style: cursor.read(),
            forest_4_tree_top_style: cursor.read(),
            corruption_tree_top_style: cursor.read(),
            jungle_tree_top_style: cursor.read(),
            snow_tree_top_style: cursor.read(),
            hallow_tree_top_style: cursor.read(),
            crimson_tree_top_style: cursor.read(),
            desert_tree_top_style: cursor.read(),
            ocean_tree_top_style: cursor.read(),
            glowing_mushroom_tree_top_style: cursor.read(),
            underworld_tree_top_style: cursor.read(),
            rain: cursor.read(),
            event_info: cursor.read(),
            event_info_2: cursor.read(),
            event_info_3: cursor.read(),
            event_info_4: cursor.read(),
            event_info_5: cursor.read(),
            event_info_6: cursor.read(),
            event_info_7: cursor.read(),
            copper_ore_tier: cursor.read(),
            iron_ore_tier: cursor.read(),
            silver_ore_tier: cursor.read(),
            gold_ore_tier: cursor.read(),
            cobalt_ore_tier: cursor.read(),
            mythril_ore_tier: cursor.read(),
            adamantite_ore_tier: cursor.read(),
            invasion_type: cursor.read(),
            lobby_id: cursor.read(),
            sandstorm_severity: cursor.read(),
        }
    }
}
