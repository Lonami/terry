mod connect_request;
mod disconnect;
mod set_user_slot;
mod player_info;
mod player_inventory_slot;
mod request_world_data;
mod world_info;
mod request_essential_tiles;
mod status;
mod send_section;
mod section_tile_frame;
mod spawn_player;
mod update_player;
mod player_active;
mod player_hp;
mod modify_tile;
mod time;
mod door_toggle;
mod send_tile_square;
mod update_item_owner;
mod npc_update;
mod strike_npc;
mod projectile_update;
mod npc_strike;
mod destroy_projectile;
mod toggle_pvp;
mod open_chest;
mod update_chest_item;
mod sync_active_chest;
mod place_chest;
mod heal_effect;
mod player_zone;
mod request_password;
mod send_password;
mod remove_item_owner;
mod set_active_npc;
mod player_item_animation;
mod player_mana;
mod mana_effect;
mod player_team;
mod request_sign;
mod update_sign;
mod set_liquid;
mod complete_connection_and_spawn;
mod update_player_buff;
mod special_npc_effect;
mod unlock;
mod add_npc_buff;
mod update_npc_buff;
mod add_player_buff;
mod update_npc_name;
mod update_good_evil;
mod play_music_item;
mod hit_switch;
mod set_npc_home;
mod spawn_boss_invasion;
mod player_dodge;
mod paint_tile;
mod paint_wall;
mod player_npc_teleport;
mod heal_other_player;
mod placeholder;
mod client_uuid;
mod get_chest_name;
mod catch_npc;
mod release_npc;
mod travelling_merchant_inventory;
mod teleportation_potion;
mod angler_quest;
mod complete_angler_quest;
mod angler_quests;
mod create_temporary_animation;
mod invasion_progress;
mod place_object;
mod sync_player_chest_index;
mod create_combat_text;
mod load_net_module;
mod set_npc_kill_count;
mod set_player_stealth;
mod quick_stash;
mod update_tile_entity;
mod place_tile_entity;
mod tweak_item;
mod place_item_frame;
mod update_item_drop_2;
mod sync_emote_bubble;
mod sync_extra_value;
mod social_handshake;
mod kill_portal;
mod player_teleport_portal;
mod player_npc_killed;
mod set_event;
mod update_minion_target;
mod npc_teleport_portal;
mod update_shield_strengths;
mod nebula_level_up;
mod moon_lord_countdown;
mod npc_shop_item;
mod gem_lock_toggle;
mod poof_of_smoke;
mod smart_text_message;
mod wired_cannon_shot;
mod mass_wire;
mod mass_consume_wire;
mod toggle_birthday_party;
mod grow_fx;
mod crystal_invasion_start;
mod crystal_invasion_wipe;
mod set_minion_target;
mod crystal_invasion_wait;
mod player_hurt;
mod player_death;
mod combat_text;
mod emoji;
mod doll_sync;
mod interact_tile_entity;
mod place_weapon_rack;
mod hat_rack_sync;
mod sync_tile_picking;
mod sync_revenge;
mod remove_revenge;
mod land_golf_ball;
mod connection_complete;
mod fish_out_npc;
mod tamper_with_npc;
mod play_legacy_sound;
mod place_food;
mod update_player_luck;
mod dead_player;
mod sync_monster_type;
mod request_npc_debuff;
mod client_synced_inventory;
mod set_as_host;

pub use connect_request::ConnectRequest;
pub use disconnect::Disconnect;
pub use set_user_slot::SetUserSlot;
pub use player_info::PlayerInfo;
pub use player_inventory_slot::PlayerInventorySlot;
pub use request_world_data::RequestWorldData;
pub use world_info::WorldInfo;
pub use request_essential_tiles::RequestEssentialTiles;
pub use status::Status;
pub use send_section::SendSection;
pub use section_tile_frame::SectionTileFrame;
pub use spawn_player::SpawnPlayer;
pub use update_player::UpdatePlayer;
pub use player_active::PlayerActive;
pub use player_hp::PlayerHP;
pub use modify_tile::ModifyTile;
pub use time::Time;
pub use door_toggle::DoorToggle;
pub use send_tile_square::SendTileSquare;
pub use update_item_owner::UpdateItemOwner;
pub use npc_update::NpcUpdate;
pub use strike_npc::StrikeNpc;
pub use projectile_update::ProjectileUpdate;
pub use npc_strike::NpcStrike;
pub use destroy_projectile::DestroyProjectile;
pub use toggle_pvp::TogglePVP;
pub use open_chest::OpenChest;
pub use update_chest_item::UpdateChestItem;
pub use sync_active_chest::SyncActiveChest;
pub use place_chest::PlaceChest;
pub use heal_effect::HealEffect;
pub use player_zone::PlayerZone;
pub use request_password::RequestPassword;
pub use send_password::SendPassword;
pub use remove_item_owner::RemoveItemOwner;
pub use set_active_npc::SetActiveNpc;
pub use player_item_animation::PlayerItemAnimation;
pub use player_mana::PlayerMana;
pub use mana_effect::ManaEffect;
pub use player_team::PlayerTeam;
pub use request_sign::RequestSign;
pub use update_sign::UpdateSign;
pub use set_liquid::SetLiquid;
pub use complete_connection_and_spawn::CompleteConnectionAndSpawn;
pub use update_player_buff::UpdatePlayerBuff;
pub use special_npc_effect::SpecialNpcEffect;
pub use unlock::Unlock;
pub use add_npc_buff::AddNpcBuff;
pub use update_npc_buff::UpdateNpcBuff;
pub use add_player_buff::AddPlayerBuff;
pub use update_npc_name::UpdateNpcName;
pub use update_good_evil::UpdateGoodEvil;
pub use play_music_item::PlayMusicItem;
pub use hit_switch::HitSwitch;
pub use set_npc_home::SetNpcHome;
pub use spawn_boss_invasion::SpawnBossInvasion;
pub use player_dodge::PlayerDodge;
pub use paint_tile::PaintTile;
pub use paint_wall::PaintWall;
pub use player_npc_teleport::PlayerNpcTeleport;
pub use heal_other_player::HealOtherPlayer;
pub use placeholder::Placeholder;
pub use client_uuid::ClientUUID;
pub use get_chest_name::GetChestName;
pub use catch_npc::CatchNpc;
pub use release_npc::ReleaseNpc;
pub use travelling_merchant_inventory::TravellingMerchantInventory;
pub use teleportation_potion::TeleportationPotion;
pub use angler_quest::AnglerQuest;
pub use complete_angler_quest::CompleteAnglerQuest;
pub use angler_quests::AnglerQuests;
pub use create_temporary_animation::CreateTemporaryAnimation;
pub use invasion_progress::InvasionProgress;
pub use place_object::PlaceObject;
pub use sync_player_chest_index::SyncPlayerChestIndex;
pub use create_combat_text::CreateCombatText;
pub use load_net_module::LoadNetModule;
pub use set_npc_kill_count::SetNpcKillCount;
pub use set_player_stealth::SetPlayerStealth;
pub use quick_stash::QuickStash;
pub use update_tile_entity::UpdateTileEntity;
pub use place_tile_entity::PlaceTileEntity;
pub use tweak_item::TweakItem;
pub use place_item_frame::PlaceItemFrame;
pub use update_item_drop_2::UpdateItemDrop2;
pub use sync_emote_bubble::SyncEmoteBubble;
pub use sync_extra_value::SyncExtraValue;
pub use social_handshake::SocialHandshake;
pub use kill_portal::KillPortal;
pub use player_teleport_portal::PlayerTeleportPortal;
pub use player_npc_killed::PlayerNpcKilled;
pub use set_event::SetEvent;
pub use update_minion_target::UpdateMinionTarget;
pub use npc_teleport_portal::NpcTeleportPortal;
pub use update_shield_strengths::UpdateShieldStrengths;
pub use nebula_level_up::NebulaLevelUp;
pub use moon_lord_countdown::MoonLordCountdown;
pub use npc_shop_item::NpcShopItem;
pub use gem_lock_toggle::GemLockToggle;
pub use poof_of_smoke::PoofofSmoke;
pub use smart_text_message::SmartTextMessage;
pub use wired_cannon_shot::WiredCannonShot;
pub use mass_wire::MassWire;
pub use mass_consume_wire::MassConsumeWire;
pub use toggle_birthday_party::ToggleBirthdayParty;
pub use grow_fx::GrowFx;
pub use crystal_invasion_start::CrystalInvasionStart;
pub use crystal_invasion_wipe::CrystalInvasionWipe;
pub use set_minion_target::SetMinionTarget;
pub use crystal_invasion_wait::CrystalInvasionWait;
pub use player_hurt::PlayerHurt;
pub use player_death::PlayerDeath;
pub use combat_text::CombatText;
pub use emoji::Emoji;
pub use doll_sync::DollSync;
pub use interact_tile_entity::InteractTileEntity;
pub use place_weapon_rack::PlaceWeaponRack;
pub use hat_rack_sync::HatRackSync;
pub use sync_tile_picking::SyncTilePicking;
pub use sync_revenge::SyncRevenge;
pub use remove_revenge::RemoveRevenge;
pub use land_golf_ball::LandGolfBall;
pub use connection_complete::ConnectionComplete;
pub use fish_out_npc::FishOutNpc;
pub use tamper_with_npc::TamperWithNpc;
pub use play_legacy_sound::PlayLegacySound;
pub use place_food::PlaceFood;
pub use update_player_luck::UpdatePlayerLuck;
pub use dead_player::DeadPlayer;
pub use sync_monster_type::SyncMonsterType;
pub use request_npc_debuff::RequestNpcDebuff;
pub use client_synced_inventory::ClientSyncedInventory;
pub use set_as_host::SetAsHost;


enum Packet {
    ConnectRequest(ConnectRequest), // 1
    Disconnect(Disconnect), // 2
    SetUserSlot(SetUserSlot), // 3
    PlayerInfo(PlayerInfo), // 4
    PlayerInventorySlot(PlayerInventorySlot), // 5
    RequestWorldData(RequestWorldData), // 6
    WorldInfo(WorldInfo), // 7
    RequestEssentialTiles(RequestEssentialTiles), // 8
    Status(Status), // 9
    SendSection(SendSection), // 10
    SectionTileFrame(SectionTileFrame), // 11
    SpawnPlayer(SpawnPlayer), // 12
    UpdatePlayer(UpdatePlayer), // 13
    PlayerActive(PlayerActive), // 14
    PlayerHP(PlayerHP), // 16
    ModifyTile(ModifyTile), // 17
    Time(Time), // 18
    DoorToggle(DoorToggle), // 19
    SendTileSquare(SendTileSquare), // 20
    UpdateItemOwner(UpdateItemOwner), // 22
    NpcUpdate(NpcUpdate), // 23
    StrikeNpc(StrikeNpc), // 24
    ProjectileUpdate(ProjectileUpdate), // 27
    NpcStrike(NpcStrike), // 28
    DestroyProjectile(DestroyProjectile), // 29
    TogglePVP(TogglePVP), // 30
    OpenChest(OpenChest), // 31
    UpdateChestItem(UpdateChestItem), // 32
    SyncActiveChest(SyncActiveChest), // 33
    PlaceChest(PlaceChest), // 34
    HealEffect(HealEffect), // 35
    PlayerZone(PlayerZone), // 36
    RequestPassword(RequestPassword), // 37
    SendPassword(SendPassword), // 38
    RemoveItemOwner(RemoveItemOwner), // 39
    SetActiveNpc(SetActiveNpc), // 40
    PlayerItemAnimation(PlayerItemAnimation), // 41
    PlayerMana(PlayerMana), // 42
    ManaEffect(ManaEffect), // 43
    PlayerTeam(PlayerTeam), // 45
    RequestSign(RequestSign), // 46
    UpdateSign(UpdateSign), // 47
    SetLiquid(SetLiquid), // 48
    CompleteConnectionAndSpawn(CompleteConnectionAndSpawn), // 49
    UpdatePlayerBuff(UpdatePlayerBuff), // 50
    SpecialNpcEffect(SpecialNpcEffect), // 51
    Unlock(Unlock), // 52
    AddNpcBuff(AddNpcBuff), // 53
    UpdateNpcBuff(UpdateNpcBuff), // 54
    AddPlayerBuff(AddPlayerBuff), // 55
    UpdateNpcName(UpdateNpcName), // 56
    UpdateGoodEvil(UpdateGoodEvil), // 57
    PlayMusicItem(PlayMusicItem), // 58
    HitSwitch(HitSwitch), // 59
    SetNpcHome(SetNpcHome), // 60
    SpawnBossInvasion(SpawnBossInvasion), // 61
    PlayerDodge(PlayerDodge), // 62
    PaintTile(PaintTile), // 63
    PaintWall(PaintWall), // 64
    PlayerNpcTeleport(PlayerNpcTeleport), // 65
    HealOtherPlayer(HealOtherPlayer), // 66
    Placeholder(Placeholder), // 67
    ClientUUID(ClientUUID), // 68
    GetChestName(GetChestName), // 69
    CatchNpc(CatchNpc), // 70
    ReleaseNpc(ReleaseNpc), // 71
    TravellingMerchantInventory(TravellingMerchantInventory), // 72
    TeleportationPotion(TeleportationPotion), // 73
    AnglerQuest(AnglerQuest), // 74
    CompleteAnglerQuest(CompleteAnglerQuest), // 75
    AnglerQuests(AnglerQuests), // 76
    CreateTemporaryAnimation(CreateTemporaryAnimation), // 77
    InvasionProgress(InvasionProgress), // 78
    PlaceObject(PlaceObject), // 79
    SyncPlayerChestIndex(SyncPlayerChestIndex), // 80
    CreateCombatText(CreateCombatText), // 81
    LoadNetModule(LoadNetModule), // 82
    SetNpcKillCount(SetNpcKillCount), // 83
    SetPlayerStealth(SetPlayerStealth), // 84
    QuickStash(QuickStash), // 85
    UpdateTileEntity(UpdateTileEntity), // 86
    PlaceTileEntity(PlaceTileEntity), // 87
    TweakItem(TweakItem), // 88
    PlaceItemFrame(PlaceItemFrame), // 89
    UpdateItemDrop2(UpdateItemDrop2), // 90
    SyncEmoteBubble(SyncEmoteBubble), // 91
    SyncExtraValue(SyncExtraValue), // 92
    SocialHandshake(SocialHandshake), // 93
    KillPortal(KillPortal), // 95
    PlayerTeleportPortal(PlayerTeleportPortal), // 96
    PlayerNpcKilled(PlayerNpcKilled), // 97
    SetEvent(SetEvent), // 98
    UpdateMinionTarget(UpdateMinionTarget), // 99
    NpcTeleportPortal(NpcTeleportPortal), // 100
    UpdateShieldStrengths(UpdateShieldStrengths), // 101
    NebulaLevelUp(NebulaLevelUp), // 102
    MoonLordCountdown(MoonLordCountdown), // 103
    NpcShopItem(NpcShopItem), // 104
    GemLockToggle(GemLockToggle), // 105
    PoofofSmoke(PoofofSmoke), // 106
    SmartTextMessage(SmartTextMessage), // 107
    WiredCannonShot(WiredCannonShot), // 108
    MassWire(MassWire), // 109
    MassConsumeWire(MassConsumeWire), // 110
    ToggleBirthdayParty(ToggleBirthdayParty), // 111
    GrowFx(GrowFx), // 112
    CrystalInvasionStart(CrystalInvasionStart), // 113
    CrystalInvasionWipe(CrystalInvasionWipe), // 114
    SetMinionTarget(SetMinionTarget), // 115
    CrystalInvasionWait(CrystalInvasionWait), // 116
    PlayerHurt(PlayerHurt), // 117
    PlayerDeath(PlayerDeath), // 118
    CombatText(CombatText), // 119
    Emoji(Emoji), // 120
    DollSync(DollSync), // 121
    InteractTileEntity(InteractTileEntity), // 122
    PlaceWeaponRack(PlaceWeaponRack), // 123
    HatRackSync(HatRackSync), // 124
    SyncTilePicking(SyncTilePicking), // 125
    SyncRevenge(SyncRevenge), // 126
    RemoveRevenge(RemoveRevenge), // 127
    LandGolfBall(LandGolfBall), // 128
    ConnectionComplete(ConnectionComplete), // 129
    FishOutNpc(FishOutNpc), // 130
    TamperWithNpc(TamperWithNpc), // 131
    PlayLegacySound(PlayLegacySound), // 132
    PlaceFood(PlaceFood), // 133
    UpdatePlayerLuck(UpdatePlayerLuck), // 134
    DeadPlayer(DeadPlayer), // 135
    SyncMonsterType(SyncMonsterType), // 136
    RequestNpcDebuff(RequestNpcDebuff), // 137
    ClientSyncedInventory(ClientSyncedInventory), // 138
    SetAsHost(SetAsHost), // 139
}
