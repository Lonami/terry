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
mod strike_npcwith_held_item;
mod projectile_update;
mod npc_strike;
mod destroy_projectile;
mod toggle_p_v_p;
mod open_chest;
mod update_chest_item;
mod sync_active_chest;
mod placechest;
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
mod npc_home_update;
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
mod complete_angler_quest_today;
mod number_of_angler_quests_completed;
mod create_temporary_animation;
mod report_invasion_progress;
mod place_object;
mod sync_player_chest_index;
mod create_combat_text;
mod load_net_module;
mod set_npc_kill_count;
mod set_player_stealth;
mod force_item_into_nearest_chest;
mod update_tile_entity;
mod place_tile_entity;
mod tweak_item;
mod place_item_frame;
mod update_item_drop_2;
mod sync_emote_bubble;
mod sync_extra_value;
mod social_handshake;
mod deprecated;
mod kill_portal;
mod player_teleport_portal;
mod notify_player_npc_killed;
mod notify_player_of_event;
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
mod mass_wire_operation;
mod mass_wire_operation_consume;
mod toggle_birthday_party;
mod growfx;
mod crystalinvasionstart;
mod crystalinvasionwipeall;
mod minionattacktargetupdate;
mod crystalinvasionsendwaittime;
mod playerhurtv2;
mod playerdeathv2;
mod combattextstring;
mod emoji;
mod tedisplaydollitemsync;
mod requesttileentityinteraction;
mod weaponsracktryplacing;
mod tehatrackitemsync;
mod synctilepicking;
mod syncrevengemarker;
mod removerevengemarker;
mod landgolfballincup;
mod finishedconnectingtoserver;
mod fishoutnpc;
mod tamperwithnpc;
mod playlegacysound;
mod foodplattertryplacing;
mod updateplayerluckfactors;
mod deadplayer;
mod synccavernmonstertype;
mod requestnpcbuffremoval;
mod clientfinishedinventorychangesonthistick;
mod setcountsashostforgameplay;

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
pub use npc_update::NPCUpdate;
pub use strike_npcwith_held_item::StrikeNPCwithHeldItem;
pub use projectile_update::ProjectileUpdate;
pub use npc_strike::NPCStrike;
pub use destroy_projectile::DestroyProjectile;
pub use toggle_p_v_p::TogglePVP;
pub use open_chest::OpenChest;
pub use update_chest_item::UpdateChestItem;
pub use sync_active_chest::SyncActiveChest;
pub use placechest::PlaceChest;
pub use heal_effect::HealEffect;
pub use player_zone::PlayerZone;
pub use request_password::RequestPassword;
pub use send_password::SendPassword;
pub use remove_item_owner::RemoveItemOwner;
pub use set_active_npc::SetActiveNPC;
pub use player_item_animation::PlayerItemAnimation;
pub use player_mana::PlayerMana;
pub use mana_effect::ManaEffect;
pub use player_team::PlayerTeam;
pub use request_sign::RequestSign;
pub use update_sign::UpdateSign;
pub use set_liquid::SetLiquid;
pub use complete_connection_and_spawn::CompleteConnectionandSpawn;
pub use update_player_buff::UpdatePlayerBuff;
pub use special_npc_effect::SpecialNPCEffect;
pub use unlock::Unlock;
pub use add_npc_buff::AddNPCBuff;
pub use update_npc_buff::UpdateNPCBuff;
pub use add_player_buff::AddPlayerBuff;
pub use update_npc_name::UpdateNPCName;
pub use update_good_evil::UpdateGoodEvil;
pub use play_music_item::PlayMusicItem;
pub use hit_switch::HitSwitch;
pub use npc_home_update::NPCHomeUpdate;
pub use spawn_boss_invasion::SpawnBossInvasion;
pub use player_dodge::PlayerDodge;
pub use paint_tile::PaintTile;
pub use paint_wall::PaintWall;
pub use player_npc_teleport::PlayerNPCTeleport;
pub use heal_other_player::HealOtherPlayer;
pub use placeholder::Placeholder;
pub use client_uuid::ClientUUID;
pub use get_chest_name::GetChestName;
pub use catch_npc::CatchNPC;
pub use release_npc::ReleaseNPC;
pub use travelling_merchant_inventory::TravellingMerchantInventory;
pub use teleportation_potion::TeleportationPotion;
pub use angler_quest::AnglerQuest;
pub use complete_angler_quest_today::CompleteAnglerQuestToday;
pub use number_of_angler_quests_completed::NumberOfAnglerQuestsCompleted;
pub use create_temporary_animation::CreateTemporaryAnimation;
pub use report_invasion_progress::ReportInvasionProgress;
pub use place_object::PlaceObject;
pub use sync_player_chest_index::SyncPlayerChestIndex;
pub use create_combat_text::CreateCombatText;
pub use load_net_module::LoadNetModule;
pub use set_npc_kill_count::SetNPCKillCount;
pub use set_player_stealth::SetPlayerStealth;
pub use force_item_into_nearest_chest::ForceItemIntoNearestChest;
pub use update_tile_entity::UpdateTileEntity;
pub use place_tile_entity::PlaceTileEntity;
pub use tweak_item::TweakItem;
pub use place_item_frame::PlaceItemFrame;
pub use update_item_drop_2::UpdateItemDrop2;
pub use sync_emote_bubble::SyncEmoteBubble;
pub use sync_extra_value::SyncExtraValue;
pub use social_handshake::SocialHandshake;
pub use deprecated::Deprecated;
pub use kill_portal::KillPortal;
pub use player_teleport_portal::PlayerTeleportPortal;
pub use notify_player_npc_killed::NotifyPlayerNPCKilled;
pub use notify_player_of_event::NotifyPlayerOfEvent;
pub use update_minion_target::UpdateMinionTarget;
pub use npc_teleport_portal::NPCTeleportPortal;
pub use update_shield_strengths::UpdateShieldStrengths;
pub use nebula_level_up::NebulaLevelUp;
pub use moon_lord_countdown::MoonLordCountdown;
pub use npc_shop_item::NPCShopItem;
pub use gem_lock_toggle::GemLockToggle;
pub use poof_of_smoke::PoofofSmoke;
pub use smart_text_message::SmartTextMessage;
pub use wired_cannon_shot::WiredCannonShot;
pub use mass_wire_operation::MassWireOperation;
pub use mass_wire_operation_consume::MassWireOperationConsume;
pub use toggle_birthday_party::ToggleBirthdayParty;
pub use growfx::GrowFX;
pub use crystalinvasionstart::CrystalInvasionStart;
pub use crystalinvasionwipeall::CrystalInvasionWipeAll;
pub use minionattacktargetupdate::MinionAttackTargetUpdate;
pub use crystalinvasionsendwaittime::CrystalInvasionSendWaitTime;
pub use playerhurtv2::PlayerHurtV2;
pub use playerdeathv2::PlayerDeathV2;
pub use combattextstring::CombatTextString;
pub use emoji::Emoji;
pub use tedisplaydollitemsync::TEDisplayDollItemSync;
pub use requesttileentityinteraction::RequestTileEntityInteraction;
pub use weaponsracktryplacing::WeaponsRackTryPlacing;
pub use tehatrackitemsync::TEHatRackItemSync;
pub use synctilepicking::SyncTilePicking;
pub use syncrevengemarker::SyncRevengeMarker;
pub use removerevengemarker::RemoveRevengeMarker;
pub use landgolfballincup::LandGolfBallInCup;
pub use finishedconnectingtoserver::FinishedConnectingToServer;
pub use fishoutnpc::FishOutNPC;
pub use tamperwithnpc::TamperWithNPC;
pub use playlegacysound::PlayLegacySound;
pub use foodplattertryplacing::FoodPlatterTryPlacing;
pub use updateplayerluckfactors::UpdatePlayerLuckFactors;
pub use deadplayer::DeadPlayer;
pub use synccavernmonstertype::SyncCavernMonsterType;
pub use requestnpcbuffremoval::RequestNPCBuffRemoval;
pub use clientfinishedinventorychangesonthistick::ClientFinishedInventoryChangesOnThisTick;
pub use setcountsashostforgameplay::SetCountsAsHostForGameplay;

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
    NPCUpdate(NPCUpdate), // 23
    StrikeNPCwithHeldItem(StrikeNPCwithHeldItem), // 24
    ProjectileUpdate(ProjectileUpdate), // 27
    NPCStrike(NPCStrike), // 28
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
    SetActiveNPC(SetActiveNPC), // 40
    PlayerItemAnimation(PlayerItemAnimation), // 41
    PlayerMana(PlayerMana), // 42
    ManaEffect(ManaEffect), // 43
    PlayerTeam(PlayerTeam), // 45
    RequestSign(RequestSign), // 46
    UpdateSign(UpdateSign), // 47
    SetLiquid(SetLiquid), // 48
    CompleteConnectionandSpawn(CompleteConnectionandSpawn), // 49
    UpdatePlayerBuff(UpdatePlayerBuff), // 50
    SpecialNPCEffect(SpecialNPCEffect), // 51
    Unlock(Unlock), // 52
    AddNPCBuff(AddNPCBuff), // 53
    UpdateNPCBuff(UpdateNPCBuff), // 54
    AddPlayerBuff(AddPlayerBuff), // 55
    UpdateNPCName(UpdateNPCName), // 56
    UpdateGoodEvil(UpdateGoodEvil), // 57
    PlayMusicItem(PlayMusicItem), // 58
    HitSwitch(HitSwitch), // 59
    NPCHomeUpdate(NPCHomeUpdate), // 60
    SpawnBossInvasion(SpawnBossInvasion), // 61
    PlayerDodge(PlayerDodge), // 62
    PaintTile(PaintTile), // 63
    PaintWall(PaintWall), // 64
    PlayerNPCTeleport(PlayerNPCTeleport), // 65
    HealOtherPlayer(HealOtherPlayer), // 66
    Placeholder(Placeholder), // 67
    ClientUUID(ClientUUID), // 68
    GetChestName(GetChestName), // 69
    CatchNPC(CatchNPC), // 70
    ReleaseNPC(ReleaseNPC), // 71
    TravellingMerchantInventory(TravellingMerchantInventory), // 72
    TeleportationPotion(TeleportationPotion), // 73
    AnglerQuest(AnglerQuest), // 74
    CompleteAnglerQuestToday(CompleteAnglerQuestToday), // 75
    NumberOfAnglerQuestsCompleted(NumberOfAnglerQuestsCompleted), // 76
    CreateTemporaryAnimation(CreateTemporaryAnimation), // 77
    ReportInvasionProgress(ReportInvasionProgress), // 78
    PlaceObject(PlaceObject), // 79
    SyncPlayerChestIndex(SyncPlayerChestIndex), // 80
    CreateCombatText(CreateCombatText), // 81
    LoadNetModule(LoadNetModule), // 82
    SetNPCKillCount(SetNPCKillCount), // 83
    SetPlayerStealth(SetPlayerStealth), // 84
    ForceItemIntoNearestChest(ForceItemIntoNearestChest), // 85
    UpdateTileEntity(UpdateTileEntity), // 86
    PlaceTileEntity(PlaceTileEntity), // 87
    TweakItem(TweakItem), // 88
    PlaceItemFrame(PlaceItemFrame), // 89
    UpdateItemDrop2(UpdateItemDrop2), // 90
    SyncEmoteBubble(SyncEmoteBubble), // 91
    SyncExtraValue(SyncExtraValue), // 92
    SocialHandshake(SocialHandshake), // 93
    Deprecated(Deprecated), // 94
    KillPortal(KillPortal), // 95
    PlayerTeleportPortal(PlayerTeleportPortal), // 96
    NotifyPlayerNPCKilled(NotifyPlayerNPCKilled), // 97
    NotifyPlayerOfEvent(NotifyPlayerOfEvent), // 98
    UpdateMinionTarget(UpdateMinionTarget), // 99
    NPCTeleportPortal(NPCTeleportPortal), // 100
    UpdateShieldStrengths(UpdateShieldStrengths), // 101
    NebulaLevelUp(NebulaLevelUp), // 102
    MoonLordCountdown(MoonLordCountdown), // 103
    NPCShopItem(NPCShopItem), // 104
    GemLockToggle(GemLockToggle), // 105
    PoofofSmoke(PoofofSmoke), // 106
    SmartTextMessage(SmartTextMessage), // 107
    WiredCannonShot(WiredCannonShot), // 108
    MassWireOperation(MassWireOperation), // 109
    MassWireOperationConsume(MassWireOperationConsume), // 110
    ToggleBirthdayParty(ToggleBirthdayParty), // 111
    GrowFX(GrowFX), // 112
    CrystalInvasionStart(CrystalInvasionStart), // 113
    CrystalInvasionWipeAll(CrystalInvasionWipeAll), // 114
    MinionAttackTargetUpdate(MinionAttackTargetUpdate), // 115
    CrystalInvasionSendWaitTime(CrystalInvasionSendWaitTime), // 116
    PlayerHurtV2(PlayerHurtV2), // 117
    PlayerDeathV2(PlayerDeathV2), // 118
    CombatTextString(CombatTextString), // 119
    Emoji(Emoji), // 120
    TEDisplayDollItemSync(TEDisplayDollItemSync), // 121
    RequestTileEntityInteraction(RequestTileEntityInteraction), // 122
    WeaponsRackTryPlacing(WeaponsRackTryPlacing), // 123
    TEHatRackItemSync(TEHatRackItemSync), // 124
    SyncTilePicking(SyncTilePicking), // 125
    SyncRevengeMarker(SyncRevengeMarker), // 126
    RemoveRevengeMarker(RemoveRevengeMarker), // 127
    LandGolfBallInCup(LandGolfBallInCup), // 128
    FinishedConnectingToServer(FinishedConnectingToServer), // 129
    FishOutNPC(FishOutNPC), // 130
    TamperWithNPC(TamperWithNPC), // 131
    PlayLegacySound(PlayLegacySound), // 132
    FoodPlatterTryPlacing(FoodPlatterTryPlacing), // 133
    UpdatePlayerLuckFactors(UpdatePlayerLuckFactors), // 134
    DeadPlayer(DeadPlayer), // 135
    SyncCavernMonsterType(SyncCavernMonsterType), // 136
    RequestNPCBuffRemoval(RequestNPCBuffRemoval), // 137
    ClientFinishedInventoryChangesOnThisTick(ClientFinishedInventoryChangesOnThisTick), // 138
    SetCountsAsHostForGameplay(SetCountsAsHostForGameplay), // 139
}
