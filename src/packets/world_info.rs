use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Contains the basic information about the world.
///
/// Direction: Server to Client.
#[derive(Debug)]
pub struct WorldInfo {
    pub time: i32,
    pub day_flags: u8,
    pub moon_phase: u8,
    pub horizontal_tiles: u16,
    pub vertical_tiles: u16,
    pub spawn_x: u16,
    pub spawn_y: u16,
    pub surface_layer: u16,
    pub rock_layer: u16,
    pub world: i32,
    pub world_name: String,
    pub game_mode: u8,
    pub world_uuid: [u8; 16],
    pub world_gen_version: u64,
    pub moon: u8,
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
    pub wind_speed: u32,
    pub cloud_number: u8,
    pub trees: [i32; 3],
    pub tree_styles: [u8; 4],
    pub cave_backs: [i32; 3],
    pub cave_back_styles: [u8; 4],
    pub forest_tree_styles: [i32; 4],
    pub corruption_tree_style: i32,
    pub jungle_tree_style: i32,
    pub snow_tree_style: i32,
    pub hallow_tree_style: i32,
    pub crimson_tree_style: i32,
    pub desert_tree_style: i32,
    pub ocean_tree_style: i32,
    pub glowing_mushroom_tree_style: i32,
    pub underworld_tree_style: i32,
    pub rain: i32,
    pub event_info: [u8; 7],
    /// The respective tier indices, names and possible tile IDs are:
    ///
    /// * Tier 0: Copper (tile ID 7 or 166)
    /// * Tier 1: Iron (tile ID 6 or 167)
    /// * Tier 2: Silver (tile ID 9 or 168)
    /// * Tier 3: Gold (tile ID 8 or 169)
    /// * Tier 4: Cobalt (tile ID 107 or 221)
    /// * Tier 5: Mythril (tile ID 108 or 222)
    /// * Tier 6: Adamantite (tile ID 111 or 223)
    pub ore_tiers_tiles: [u16; 7],
    pub invasion_type: u8,
    pub lobby_id: u64,
    pub sandstorm_severity: i32,
}

impl WorldInfo {
    fn day_time(&self) -> bool {
        self.day_flags & 0x01 != 0
    }

    fn blood_moon(&self) -> bool {
        self.day_flags & 0x02 != 0
    }

    fn eclipse(&self) -> bool {
        self.day_flags & 0x04 != 0
    }

    fn shadow_orb_smashed(&self) -> bool {
        self.event_info[0] & 0x01 != 0
    }

    fn downed_boss_1(&self) -> bool {
        self.event_info[0] & 0x02 != 0
    }

    fn downed_boss_2(&self) -> bool {
        self.event_info[0] & 0x04 != 0
    }

    fn downed_boss_3(&self) -> bool {
        self.event_info[0] & 0x08 != 0
    }

    fn hard_mode(&self) -> bool {
        self.event_info[0] & 0x10 != 0
    }

    fn downed_clown(&self) -> bool {
        self.event_info[0] & 0x20 != 0
    }

    fn server_side_character(&self) -> bool {
        self.event_info[0] & 0x40 != 0
    }

    fn downed_plant_boss(&self) -> bool {
        self.event_info[0] & 0x80 != 0
    }

    fn downed_mech_boss(&self) -> bool {
        self.event_info[1] & 0x01 != 0
    }

    fn downed_mech_boss_2(&self) -> bool {
        self.event_info[1] & 0x02 != 0
    }

    fn downed_mech_boss_3(&self) -> bool {
        self.event_info[1] & 0x04 != 0
    }

    fn downed_mech_boss_any(&self) -> bool {
        self.event_info[1] & 0x08 != 0
    }

    fn cloud_bg(&self) -> bool {
        self.event_info[1] & 0x10 != 0
    }

    fn crimson(&self) -> bool {
        self.event_info[1] & 0x20 != 0
    }

    fn pumpkin_moon(&self) -> bool {
        self.event_info[1] & 0x40 != 0
    }

    fn snow_moon(&self) -> bool {
        self.event_info[1] & 0x80 != 0
    }

    fn expert_mode(&self) -> bool {
        self.event_info[2] & 0x01 != 0
    }

    fn fast_forward_time(&self) -> bool {
        self.event_info[2] & 0x02 != 0
    }

    fn slime_rain(&self) -> bool {
        self.event_info[2] & 0x04 != 0
    }

    fn downed_king_slime(&self) -> bool {
        self.event_info[2] & 0x08 != 0
    }

    fn downed_queen_bee(&self) -> bool {
        self.event_info[2] & 0x10 != 0
    }

    fn downed_fishron(&self) -> bool {
        self.event_info[2] & 0x20 != 0
    }

    fn downed_martians(&self) -> bool {
        self.event_info[2] & 0x40 != 0
    }

    fn downed_ancient_cultist(&self) -> bool {
        self.event_info[2] & 0x80 != 0
    }

    fn downed_moon_lord(&self) -> bool {
        self.event_info[3] & 0x01 != 0
    }

    fn downed_pumking(&self) -> bool {
        self.event_info[3] & 0x02 != 0
    }

    fn downed_mourning_wood(&self) -> bool {
        self.event_info[3] & 0x04 != 0
    }

    fn downed_ice_queen(&self) -> bool {
        self.event_info[3] & 0x08 != 0
    }

    fn downed_santank(&self) -> bool {
        self.event_info[3] & 0x10 != 0
    }

    fn downed_everscream(&self) -> bool {
        self.event_info[3] & 0x20 != 0
    }

    fn downed_golem(&self) -> bool {
        self.event_info[3] & 0x40 != 0
    }

    fn birthday_party(&self) -> bool {
        self.event_info[3] & 0x80 != 0
    }

    fn downed_pirates(&self) -> bool {
        self.event_info[4] & 0x01 != 0
    }

    fn downed_frost_legion(&self) -> bool {
        self.event_info[4] & 0x02 != 0
    }

    fn downed_goblins(&self) -> bool {
        self.event_info[4] & 0x04 != 0
    }

    fn sandstorm(&self) -> bool {
        self.event_info[4] & 0x08 != 0
    }

    fn dd2_event(&self) -> bool {
        self.event_info[4] & 0x10 != 0
    }

    fn downed_dd2_tier_1(&self) -> bool {
        self.event_info[4] & 0x20 != 0
    }

    fn downed_dd2_tier_2(&self) -> bool {
        self.event_info[4] & 0x40 != 0
    }

    fn downed_dd2_tier_3(&self) -> bool {
        self.event_info[4] & 0x80 != 0
    }

    fn combat_book_used(&self) -> bool {
        self.event_info[5] & 0x01 != 0
    }

    fn manual_lanterns(&self) -> bool {
        self.event_info[5] & 0x02 != 0
    }

    fn downed_solar_tower(&self) -> bool {
        self.event_info[5] & 0x04 != 0
    }

    fn downed_vortex_tower(&self) -> bool {
        self.event_info[5] & 0x08 != 0
    }

    fn downed_nebula_tower(&self) -> bool {
        self.event_info[5] & 0x10 != 0
    }

    fn downed_stardust_tower(&self) -> bool {
        self.event_info[5] & 0x20 != 0
    }

    fn force_halloween(&self) -> bool {
        self.event_info[5] & 0x40 != 0
    }

    fn force_xmas(&self) -> bool {
        self.event_info[5] & 0x80 != 0
    }

    fn bought_cat(&self) -> bool {
        self.event_info[6] & 0x01 != 0
    }

    fn bought_dog(&self) -> bool {
        self.event_info[6] & 0x02 != 0
    }

    fn bought_bunny(&self) -> bool {
        self.event_info[6] & 0x04 != 0
    }

    fn free_cake(&self) -> bool {
        self.event_info[6] & 0x08 != 0
    }

    fn drunk_world(&self) -> bool {
        self.event_info[6] & 0x10 != 0
    }

    fn downed_empress_of_light(&self) -> bool {
        self.event_info[6] & 0x20 != 0
    }

    fn downed_queen_slime(&self) -> bool {
        self.event_info[6] & 0x40 != 0
    }
}

impl PacketBody for WorldInfo {
    const TAG: u8 = 7;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.time);
        cursor.write(&self.day_flags);
        cursor.write(&self.moon_phase);
        cursor.write(&self.horizontal_tiles);
        cursor.write(&self.vertical_tiles);
        cursor.write(&self.spawn_x);
        cursor.write(&self.spawn_y);
        cursor.write(&self.surface_layer);
        cursor.write(&self.rock_layer);
        cursor.write(&self.world);
        cursor.write(&self.world_name);
        cursor.write(&self.game_mode);
        cursor.write(&self.world_uuid);
        cursor.write(&self.world_gen_version);
        cursor.write(&self.moon);
        cursor.write(&self.tree_background);
        cursor.write(&self.corruption_background);
        cursor.write(&self.jungle_background);
        cursor.write(&self.snow_background);
        cursor.write(&self.hallow_background);
        cursor.write(&self.crimson_background);
        cursor.write(&self.desert_background);
        cursor.write(&self.ocean_background);
        cursor.write(&self.unknown_background);
        cursor.write(&self.ice_back_style);
        cursor.write(&self.jungle_back_style);
        cursor.write(&self.hell_back_style);
        cursor.write(&self.wind_speed);
        cursor.write(&self.cloud_number);
        cursor.write(&self.trees);
        cursor.write(&self.tree_styles);
        cursor.write(&self.cave_backs);
        cursor.write(&self.cave_back_styles);
        cursor.write(&self.forest_tree_styles);
        cursor.write(&self.corruption_tree_style);
        cursor.write(&self.jungle_tree_style);
        cursor.write(&self.snow_tree_style);
        cursor.write(&self.hallow_tree_style);
        cursor.write(&self.crimson_tree_style);
        cursor.write(&self.desert_tree_style);
        cursor.write(&self.ocean_tree_style);
        cursor.write(&self.glowing_mushroom_tree_style);
        cursor.write(&self.underworld_tree_style);
        cursor.write(&self.rain);
        cursor.write(&self.event_info);
        cursor.write(&self.ore_tiers_tiles);
        cursor.write(&self.invasion_type);
        cursor.write(&self.lobby_id);
        cursor.write(&self.sandstorm_severity);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            time: cursor.read(),
            day_flags: cursor.read(),
            moon_phase: cursor.read(),
            horizontal_tiles: cursor.read(),
            vertical_tiles: cursor.read(),
            spawn_x: cursor.read(),
            spawn_y: cursor.read(),
            surface_layer: cursor.read(),
            rock_layer: cursor.read(),
            world: cursor.read(),
            world_name: cursor.read(),
            game_mode: cursor.read(),
            world_uuid: cursor.read(),
            world_gen_version: cursor.read(),
            moon: cursor.read(),
            tree_background: cursor.read(),
            corruption_background: cursor.read(),
            jungle_background: cursor.read(),
            snow_background: cursor.read(),
            hallow_background: cursor.read(),
            crimson_background: cursor.read(),
            desert_background: cursor.read(),
            ocean_background: cursor.read(),
            unknown_background: cursor.read(),
            ice_back_style: cursor.read(),
            jungle_back_style: cursor.read(),
            hell_back_style: cursor.read(),
            wind_speed: cursor.read(),
            cloud_number: cursor.read(),
            trees: cursor.read(),
            tree_styles: cursor.read(),
            cave_backs: cursor.read(),
            cave_back_styles: cursor.read(),
            forest_tree_styles: cursor.read(),
            corruption_tree_style: cursor.read(),
            jungle_tree_style: cursor.read(),
            snow_tree_style: cursor.read(),
            hallow_tree_style: cursor.read(),
            crimson_tree_style: cursor.read(),
            desert_tree_style: cursor.read(),
            ocean_tree_style: cursor.read(),
            glowing_mushroom_tree_style: cursor.read(),
            underworld_tree_style: cursor.read(),
            rain: cursor.read(),
            event_info: cursor.read(),
            ore_tiers_tiles: cursor.read(),
            invasion_type: cursor.read(),
            lobby_id: cursor.read(),
            sandstorm_severity: cursor.read(),
        }
    }
}
