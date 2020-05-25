mod add_npc_buff;
mod add_player_buff;
mod angler_quest;
mod angler_quests;
mod catch_npc;
mod client_synced_inventory;
mod client_uuid;
mod combat_text;
mod complete_angler_quest;
mod complete_connection_and_spawn;
mod connect;
mod connection_complete;
mod create_combat_text;
mod create_temporary_animation;
mod crystal_invasion_start;
mod crystal_invasion_wait;
mod crystal_invasion_wipe;
mod dead_player;
mod destroy_projectile;
mod disconnect;
mod doll_sync;
mod door_toggle;
mod emoji;
mod fish_out_npc;
mod gem_lock_toggle;
mod get_chest_name;
mod grow_fx;
mod hat_rack_sync;
mod heal_effect;
mod heal_other_player;
mod hit_switch;
mod interact_tile_entity;
mod invasion_progress;
mod kill_portal;
mod land_golf_ball;
mod load_net_module;
mod mana_effect;
mod mass_consume_wire;
mod mass_wire;
mod modify_tile;
mod moon_lord_countdown;
mod nebula_level_up;
mod npc_shop_item;
mod npc_strike;
mod npc_teleport_portal;
mod npc_update;
mod open_chest;
mod paint_tile;
mod paint_wall;
mod place_chest;
mod place_food;
mod place_item_frame;
mod place_object;
mod place_tile_entity;
mod place_weapon_rack;
mod placeholder;
mod play_legacy_sound;
mod play_music_item;
mod player_active;
mod player_death;
mod player_dodge;
mod player_hp;
mod player_hurt;
mod player_info;
mod player_inventory_slot;
mod player_item_animation;
mod player_mana;
mod player_npc_killed;
mod player_npc_teleport;
mod player_team;
mod player_teleport_portal;
mod player_zone;
mod poof_of_smoke;
mod projectile_update;
mod quick_stash;
mod release_npc;
mod remove_item_owner;
mod remove_revenge;
mod request_essential_tiles;
mod request_npc_debuff;
mod request_password;
mod request_sign;
mod request_world_data;
mod section_tile_frame;
mod send_password;
mod send_section;
mod send_tile_square;
mod set_active_npc;
mod set_as_host;
mod set_event;
mod set_liquid;
mod set_minion_target;
mod set_npc_home;
mod set_npc_kill_count;
mod set_player_stealth;
mod set_user_slot;
mod smart_text_message;
mod social_handshake;
mod spawn_boss_invasion;
mod spawn_player;
mod special_npc_effect;
mod status;
mod strike_npc;
mod sync_active_chest;
mod sync_emote_bubble;
mod sync_extra_value;
mod sync_monster_type;
mod sync_player_chest_index;
mod sync_revenge;
mod sync_tile_picking;
mod tamper_with_npc;
mod teleportation_potion;
mod time;
mod toggle_birthday_party;
mod toggle_pvp;
mod travelling_merchant_inventory;
mod tweak_item;
mod unlock;
mod update_chest_item;
mod update_good_evil;
mod update_item_drop_2;
mod update_item_owner;
mod update_minion_target;
mod update_npc_buff;
mod update_npc_name;
mod update_player;
mod update_player_buff;
mod update_player_luck;
mod update_shield_strengths;
mod update_sign;
mod update_tile_entity;
mod wired_cannon_shot;
mod world_info;

pub use add_npc_buff::AddNpcBuff;
pub use add_player_buff::AddPlayerBuff;
pub use angler_quest::AnglerQuest;
pub use angler_quests::AnglerQuests;
pub use catch_npc::CatchNpc;
pub use client_synced_inventory::ClientSyncedInventory;
pub use client_uuid::ClientUuid;
pub use combat_text::CombatText;
pub use complete_angler_quest::CompleteAnglerQuest;
pub use complete_connection_and_spawn::CompleteConnectionAndSpawn;
pub use connect::Connect;
pub use connection_complete::ConnectionComplete;
pub use create_combat_text::CreateCombatText;
pub use create_temporary_animation::CreateTemporaryAnimation;
pub use crystal_invasion_start::CrystalInvasionStart;
pub use crystal_invasion_wait::CrystalInvasionWait;
pub use crystal_invasion_wipe::CrystalInvasionWipe;
pub use dead_player::DeadPlayer;
pub use destroy_projectile::DestroyProjectile;
pub use disconnect::Disconnect;
pub use doll_sync::DollSync;
pub use door_toggle::DoorToggle;
pub use emoji::Emoji;
pub use fish_out_npc::FishOutNpc;
pub use gem_lock_toggle::GemLockToggle;
pub use get_chest_name::GetChestName;
pub use grow_fx::GrowFx;
pub use hat_rack_sync::HatRackSync;
pub use heal_effect::HealEffect;
pub use heal_other_player::HealOtherPlayer;
pub use hit_switch::HitSwitch;
pub use interact_tile_entity::InteractTileEntity;
pub use invasion_progress::InvasionProgress;
pub use kill_portal::KillPortal;
pub use land_golf_ball::LandGolfBall;
pub use load_net_module::LoadNetModule;
pub use mana_effect::ManaEffect;
pub use mass_consume_wire::MassConsumeWire;
pub use mass_wire::MassWire;
pub use modify_tile::ModifyTile;
pub use moon_lord_countdown::MoonLordCountdown;
pub use nebula_level_up::NebulaLevelUp;
pub use npc_shop_item::NpcShopItem;
pub use npc_strike::NpcStrike;
pub use npc_teleport_portal::NpcTeleportPortal;
pub use npc_update::NpcUpdate;
pub use open_chest::OpenChest;
pub use paint_tile::PaintTile;
pub use paint_wall::PaintWall;
pub use place_chest::PlaceChest;
pub use place_food::PlaceFood;
pub use place_item_frame::PlaceItemFrame;
pub use place_object::PlaceObject;
pub use place_tile_entity::PlaceTileEntity;
pub use place_weapon_rack::PlaceWeaponRack;
pub use placeholder::Placeholder;
pub use play_legacy_sound::PlayLegacySound;
pub use play_music_item::PlayMusicItem;
pub use player_active::PlayerActive;
pub use player_death::PlayerDeath;
pub use player_dodge::PlayerDodge;
pub use player_hp::PlayerHP;
pub use player_hurt::PlayerHurt;
pub use player_info::PlayerInfo;
pub use player_inventory_slot::PlayerInventorySlot;
pub use player_item_animation::PlayerItemAnimation;
pub use player_mana::PlayerMana;
pub use player_npc_killed::PlayerNpcKilled;
pub use player_npc_teleport::PlayerNpcTeleport;
pub use player_team::PlayerTeam;
pub use player_teleport_portal::PlayerTeleportPortal;
pub use player_zone::PlayerZone;
pub use poof_of_smoke::PoofOfSmoke;
pub use projectile_update::ProjectileUpdate;
pub use quick_stash::QuickStash;
pub use release_npc::ReleaseNpc;
pub use remove_item_owner::RemoveItemOwner;
pub use remove_revenge::RemoveRevenge;
pub use request_essential_tiles::RequestEssentialTiles;
pub use request_npc_debuff::RequestNpcDebuff;
pub use request_password::RequestPassword;
pub use request_sign::RequestSign;
pub use request_world_data::RequestWorldData;
pub use section_tile_frame::SectionTileFrame;
pub use send_password::SendPassword;
pub use send_section::SendSection;
pub use send_tile_square::SendTileSquare;
pub use set_active_npc::SetActiveNpc;
pub use set_as_host::SetAsHost;
pub use set_event::SetEvent;
pub use set_liquid::SetLiquid;
pub use set_minion_target::SetMinionTarget;
pub use set_npc_home::SetNpcHome;
pub use set_npc_kill_count::SetNpcKillCount;
pub use set_player_stealth::SetPlayerStealth;
pub use set_user_slot::SetUserSlot;
pub use smart_text_message::SmartTextMessage;
pub use social_handshake::SocialHandshake;
pub use spawn_boss_invasion::SpawnBossInvasion;
pub use spawn_player::SpawnPlayer;
pub use special_npc_effect::SpecialNpcEffect;
pub use status::Status;
pub use strike_npc::StrikeNpc;
pub use sync_active_chest::SyncActiveChest;
pub use sync_emote_bubble::SyncEmoteBubble;
pub use sync_extra_value::SyncExtraValue;
pub use sync_monster_type::SyncMonsterType;
pub use sync_player_chest_index::SyncPlayerChestIndex;
pub use sync_revenge::SyncRevenge;
pub use sync_tile_picking::SyncTilePicking;
pub use tamper_with_npc::TamperWithNpc;
pub use teleportation_potion::TeleportationPotion;
pub use time::Time;
pub use toggle_birthday_party::ToggleBirthdayParty;
pub use toggle_pvp::TogglePvp;
pub use travelling_merchant_inventory::TravellingMerchantInventory;
pub use tweak_item::TweakItem;
pub use unlock::Unlock;
pub use update_chest_item::UpdateChestItem;
pub use update_good_evil::UpdateGoodEvil;
pub use update_item_drop_2::UpdateItemDrop2;
pub use update_item_owner::UpdateItemOwner;
pub use update_minion_target::UpdateMinionTarget;
pub use update_npc_buff::UpdateNpcBuff;
pub use update_npc_name::UpdateNpcName;
pub use update_player::UpdatePlayer;
pub use update_player_buff::UpdatePlayerBuff;
pub use update_player_luck::UpdatePlayerLuck;
pub use update_shield_strengths::UpdateShieldStrengths;
pub use update_sign::UpdateSign;
pub use update_tile_entity::UpdateTileEntity;
pub use wired_cannon_shot::WiredCannonShot;
pub use world_info::WorldInfo;

use crate::SliceCursor;
use std::convert::TryInto;

pub trait PacketBody: Sized {
    const TAG: u8;

    fn write_body(&self, cursor: &mut SliceCursor);

    fn from_body(cursor: &mut SliceCursor) -> Self;

    // TODO player should probably go inside the packets
    fn serialize(&self, cursor: &mut SliceCursor) {
        let length_pos = cursor.pos();
        cursor.write(&0u16); // length
        cursor.write(&Self::TAG);
        self.write_body(cursor);
        let length: u16 = (cursor.pos() - length_pos)
            .try_into()
            .expect("packet too long");
        cursor.rewrite(length_pos, &length);
    }
}

#[derive(Debug)]
pub enum Packet {
    Connect(Connect),                                         // 1
    Disconnect(Disconnect),                                   // 2
    SetUserSlot(SetUserSlot),                                 // 3
    PlayerInfo(PlayerInfo),                                   // 4
    PlayerInventorySlot(PlayerInventorySlot),                 // 5
    RequestWorldData(RequestWorldData),                       // 6
    WorldInfo(WorldInfo),                                     // 7
    RequestEssentialTiles(RequestEssentialTiles),             // 8
    Status(Status),                                           // 9
    SendSection(SendSection),                                 // 10
    SectionTileFrame(SectionTileFrame),                       // 11
    SpawnPlayer(SpawnPlayer),                                 // 12
    UpdatePlayer(UpdatePlayer),                               // 13
    PlayerActive(PlayerActive),                               // 14
    PlayerHP(PlayerHP),                                       // 16
    ModifyTile(ModifyTile),                                   // 17
    Time(Time),                                               // 18
    DoorToggle(DoorToggle),                                   // 19
    SendTileSquare(SendTileSquare),                           // 20
    UpdateItemOwner(UpdateItemOwner),                         // 22
    NpcUpdate(NpcUpdate),                                     // 23
    StrikeNpc(StrikeNpc),                                     // 24
    ProjectileUpdate(ProjectileUpdate),                       // 27
    NpcStrike(NpcStrike),                                     // 28
    DestroyProjectile(DestroyProjectile),                     // 29
    TogglePvp(TogglePvp),                                     // 30
    OpenChest(OpenChest),                                     // 31
    UpdateChestItem(UpdateChestItem),                         // 32
    SyncActiveChest(SyncActiveChest),                         // 33
    PlaceChest(PlaceChest),                                   // 34
    HealEffect(HealEffect),                                   // 35
    PlayerZone(PlayerZone),                                   // 36
    RequestPassword(RequestPassword),                         // 37
    SendPassword(SendPassword),                               // 38
    RemoveItemOwner(RemoveItemOwner),                         // 39
    SetActiveNpc(SetActiveNpc),                               // 40
    PlayerItemAnimation(PlayerItemAnimation),                 // 41
    PlayerMana(PlayerMana),                                   // 42
    ManaEffect(ManaEffect),                                   // 43
    PlayerTeam(PlayerTeam),                                   // 45
    RequestSign(RequestSign),                                 // 46
    UpdateSign(UpdateSign),                                   // 47
    SetLiquid(SetLiquid),                                     // 48
    CompleteConnectionAndSpawn(CompleteConnectionAndSpawn),   // 49
    UpdatePlayerBuff(UpdatePlayerBuff),                       // 50
    SpecialNpcEffect(SpecialNpcEffect),                       // 51
    Unlock(Unlock),                                           // 52
    AddNpcBuff(AddNpcBuff),                                   // 53
    UpdateNpcBuff(UpdateNpcBuff),                             // 54
    AddPlayerBuff(AddPlayerBuff),                             // 55
    UpdateNpcName(UpdateNpcName),                             // 56
    UpdateGoodEvil(UpdateGoodEvil),                           // 57
    PlayMusicItem(PlayMusicItem),                             // 58
    HitSwitch(HitSwitch),                                     // 59
    SetNpcHome(SetNpcHome),                                   // 60
    SpawnBossInvasion(SpawnBossInvasion),                     // 61
    PlayerDodge(PlayerDodge),                                 // 62
    PaintTile(PaintTile),                                     // 63
    PaintWall(PaintWall),                                     // 64
    PlayerNpcTeleport(PlayerNpcTeleport),                     // 65
    HealOtherPlayer(HealOtherPlayer),                         // 66
    Placeholder(Placeholder),                                 // 67
    ClientUuid(ClientUuid),                                   // 68
    GetChestName(GetChestName),                               // 69
    CatchNpc(CatchNpc),                                       // 70
    ReleaseNpc(ReleaseNpc),                                   // 71
    TravellingMerchantInventory(TravellingMerchantInventory), // 72
    TeleportationPotion(TeleportationPotion),                 // 73
    AnglerQuest(AnglerQuest),                                 // 74
    CompleteAnglerQuest(CompleteAnglerQuest),                 // 75
    AnglerQuests(AnglerQuests),                               // 76
    CreateTemporaryAnimation(CreateTemporaryAnimation),       // 77
    InvasionProgress(InvasionProgress),                       // 78
    PlaceObject(PlaceObject),                                 // 79
    SyncPlayerChestIndex(SyncPlayerChestIndex),               // 80
    CreateCombatText(CreateCombatText),                       // 81
    LoadNetModule(LoadNetModule),                             // 82
    SetNpcKillCount(SetNpcKillCount),                         // 83
    SetPlayerStealth(SetPlayerStealth),                       // 84
    QuickStash(QuickStash),                                   // 85
    UpdateTileEntity(UpdateTileEntity),                       // 86
    PlaceTileEntity(PlaceTileEntity),                         // 87
    TweakItem(TweakItem),                                     // 88
    PlaceItemFrame(PlaceItemFrame),                           // 89
    UpdateItemDrop2(UpdateItemDrop2),                         // 90
    SyncEmoteBubble(SyncEmoteBubble),                         // 91
    SyncExtraValue(SyncExtraValue),                           // 92
    SocialHandshake(SocialHandshake),                         // 93
    KillPortal(KillPortal),                                   // 95
    PlayerTeleportPortal(PlayerTeleportPortal),               // 96
    PlayerNpcKilled(PlayerNpcKilled),                         // 97
    SetEvent(SetEvent),                                       // 98
    UpdateMinionTarget(UpdateMinionTarget),                   // 99
    NpcTeleportPortal(NpcTeleportPortal),                     // 100
    UpdateShieldStrengths(UpdateShieldStrengths),             // 101
    NebulaLevelUp(NebulaLevelUp),                             // 102
    MoonLordCountdown(MoonLordCountdown),                     // 103
    NpcShopItem(NpcShopItem),                                 // 104
    GemLockToggle(GemLockToggle),                             // 105
    PoofOfSmoke(PoofOfSmoke),                                 // 106
    SmartTextMessage(SmartTextMessage),                       // 107
    WiredCannonShot(WiredCannonShot),                         // 108
    MassWire(MassWire),                                       // 109
    MassConsumeWire(MassConsumeWire),                         // 110
    ToggleBirthdayParty(ToggleBirthdayParty),                 // 111
    GrowFx(GrowFx),                                           // 112
    CrystalInvasionStart(CrystalInvasionStart),               // 113
    CrystalInvasionWipe(CrystalInvasionWipe),                 // 114
    SetMinionTarget(SetMinionTarget),                         // 115
    CrystalInvasionWait(CrystalInvasionWait),                 // 116
    PlayerHurt(PlayerHurt),                                   // 117
    PlayerDeath(PlayerDeath),                                 // 118
    CombatText(CombatText),                                   // 119
    Emoji(Emoji),                                             // 120
    DollSync(DollSync),                                       // 121
    InteractTileEntity(InteractTileEntity),                   // 122
    PlaceWeaponRack(PlaceWeaponRack),                         // 123
    HatRackSync(HatRackSync),                                 // 124
    SyncTilePicking(SyncTilePicking),                         // 125
    SyncRevenge(SyncRevenge),                                 // 126
    RemoveRevenge(RemoveRevenge),                             // 127
    LandGolfBall(LandGolfBall),                               // 128
    ConnectionComplete(ConnectionComplete),                   // 129
    FishOutNpc(FishOutNpc),                                   // 130
    TamperWithNpc(TamperWithNpc),                             // 131
    PlayLegacySound(PlayLegacySound),                         // 132
    PlaceFood(PlaceFood),                                     // 133
    UpdatePlayerLuck(UpdatePlayerLuck),                       // 134
    DeadPlayer(DeadPlayer),                                   // 135
    SyncMonsterType(SyncMonsterType),                         // 136
    RequestNpcDebuff(RequestNpcDebuff),                       // 137
    ClientSyncedInventory(ClientSyncedInventory),             // 138
    SetAsHost(SetAsHost),                                     // 139
}

impl Packet {
    pub fn from_slice(slice: &mut [u8]) -> Self {
        let mut cursor = SliceCursor::new(slice);
        let tag = cursor.read::<u8>();
        // TODO too bad packet body is not serializable
        match tag {
            Connect::TAG => Packet::Connect(Connect::from_body(&mut cursor)),
            Disconnect::TAG => Packet::Disconnect(Disconnect::from_body(&mut cursor)),
            SetUserSlot::TAG => Packet::SetUserSlot(SetUserSlot::from_body(&mut cursor)),
            PlayerInfo::TAG => Packet::PlayerInfo(PlayerInfo::from_body(&mut cursor)),
            PlayerInventorySlot::TAG => {
                Packet::PlayerInventorySlot(PlayerInventorySlot::from_body(&mut cursor))
            }
            RequestWorldData::TAG => {
                Packet::RequestWorldData(RequestWorldData::from_body(&mut cursor))
            }
            WorldInfo::TAG => Packet::WorldInfo(WorldInfo::from_body(&mut cursor)),
            RequestEssentialTiles::TAG => {
                Packet::RequestEssentialTiles(RequestEssentialTiles::from_body(&mut cursor))
            }
            Status::TAG => Packet::Status(Status::from_body(&mut cursor)),
            SendSection::TAG => Packet::SendSection(SendSection::from_body(&mut cursor)),
            SectionTileFrame::TAG => {
                Packet::SectionTileFrame(SectionTileFrame::from_body(&mut cursor))
            }
            SpawnPlayer::TAG => Packet::SpawnPlayer(SpawnPlayer::from_body(&mut cursor)),
            UpdatePlayer::TAG => Packet::UpdatePlayer(UpdatePlayer::from_body(&mut cursor)),
            PlayerActive::TAG => Packet::PlayerActive(PlayerActive::from_body(&mut cursor)),
            PlayerHP::TAG => Packet::PlayerHP(PlayerHP::from_body(&mut cursor)),
            ModifyTile::TAG => Packet::ModifyTile(ModifyTile::from_body(&mut cursor)),
            Time::TAG => Packet::Time(Time::from_body(&mut cursor)),
            DoorToggle::TAG => Packet::DoorToggle(DoorToggle::from_body(&mut cursor)),
            SendTileSquare::TAG => Packet::SendTileSquare(SendTileSquare::from_body(&mut cursor)),
            21 => Packet::UpdateItemDrop2(UpdateItemDrop2::from_body(&mut cursor)),
            UpdateItemOwner::TAG => {
                Packet::UpdateItemOwner(UpdateItemOwner::from_body(&mut cursor))
            }
            NpcUpdate::TAG => Packet::NpcUpdate(NpcUpdate::from_body(&mut cursor)),
            StrikeNpc::TAG => Packet::StrikeNpc(StrikeNpc::from_body(&mut cursor)),
            25 => panic!("tag 25 is the null packet"),
            26 => panic!("tag 26 is the null packet"),
            ProjectileUpdate::TAG => {
                Packet::ProjectileUpdate(ProjectileUpdate::from_body(&mut cursor))
            }
            NpcStrike::TAG => Packet::NpcStrike(NpcStrike::from_body(&mut cursor)),
            DestroyProjectile::TAG => {
                Packet::DestroyProjectile(DestroyProjectile::from_body(&mut cursor))
            }
            TogglePvp::TAG => Packet::TogglePvp(TogglePvp::from_body(&mut cursor)),
            OpenChest::TAG => Packet::OpenChest(OpenChest::from_body(&mut cursor)),
            UpdateChestItem::TAG => {
                Packet::UpdateChestItem(UpdateChestItem::from_body(&mut cursor))
            }
            SyncActiveChest::TAG => {
                Packet::SyncActiveChest(SyncActiveChest::from_body(&mut cursor))
            }
            PlaceChest::TAG => Packet::PlaceChest(PlaceChest::from_body(&mut cursor)),
            HealEffect::TAG => Packet::HealEffect(HealEffect::from_body(&mut cursor)),
            PlayerZone::TAG => Packet::PlayerZone(PlayerZone::from_body(&mut cursor)),
            RequestPassword::TAG => {
                Packet::RequestPassword(RequestPassword::from_body(&mut cursor))
            }
            SendPassword::TAG => Packet::SendPassword(SendPassword::from_body(&mut cursor)),
            RemoveItemOwner::TAG => {
                Packet::RemoveItemOwner(RemoveItemOwner::from_body(&mut cursor))
            }
            SetActiveNpc::TAG => Packet::SetActiveNpc(SetActiveNpc::from_body(&mut cursor)),
            PlayerItemAnimation::TAG => {
                Packet::PlayerItemAnimation(PlayerItemAnimation::from_body(&mut cursor))
            }
            PlayerMana::TAG => Packet::PlayerMana(PlayerMana::from_body(&mut cursor)),
            ManaEffect::TAG => Packet::ManaEffect(ManaEffect::from_body(&mut cursor)),
            44 => panic!("tag 44 is the null packet"),
            PlayerTeam::TAG => Packet::PlayerTeam(PlayerTeam::from_body(&mut cursor)),
            RequestSign::TAG => Packet::RequestSign(RequestSign::from_body(&mut cursor)),
            UpdateSign::TAG => Packet::UpdateSign(UpdateSign::from_body(&mut cursor)),
            SetLiquid::TAG => Packet::SetLiquid(SetLiquid::from_body(&mut cursor)),
            CompleteConnectionAndSpawn::TAG => Packet::CompleteConnectionAndSpawn(
                CompleteConnectionAndSpawn::from_body(&mut cursor),
            ),
            UpdatePlayerBuff::TAG => {
                Packet::UpdatePlayerBuff(UpdatePlayerBuff::from_body(&mut cursor))
            }
            SpecialNpcEffect::TAG => {
                Packet::SpecialNpcEffect(SpecialNpcEffect::from_body(&mut cursor))
            }
            Unlock::TAG => Packet::Unlock(Unlock::from_body(&mut cursor)),
            AddNpcBuff::TAG => Packet::AddNpcBuff(AddNpcBuff::from_body(&mut cursor)),
            UpdateNpcBuff::TAG => Packet::UpdateNpcBuff(UpdateNpcBuff::from_body(&mut cursor)),
            AddPlayerBuff::TAG => Packet::AddPlayerBuff(AddPlayerBuff::from_body(&mut cursor)),
            UpdateNpcName::TAG => Packet::UpdateNpcName(UpdateNpcName::from_body(&mut cursor)),
            UpdateGoodEvil::TAG => Packet::UpdateGoodEvil(UpdateGoodEvil::from_body(&mut cursor)),
            PlayMusicItem::TAG => Packet::PlayMusicItem(PlayMusicItem::from_body(&mut cursor)),
            HitSwitch::TAG => Packet::HitSwitch(HitSwitch::from_body(&mut cursor)),
            SetNpcHome::TAG => Packet::SetNpcHome(SetNpcHome::from_body(&mut cursor)),
            SpawnBossInvasion::TAG => {
                Packet::SpawnBossInvasion(SpawnBossInvasion::from_body(&mut cursor))
            }
            PlayerDodge::TAG => Packet::PlayerDodge(PlayerDodge::from_body(&mut cursor)),
            PaintTile::TAG => Packet::PaintTile(PaintTile::from_body(&mut cursor)),
            PaintWall::TAG => Packet::PaintWall(PaintWall::from_body(&mut cursor)),
            PlayerNpcTeleport::TAG => {
                Packet::PlayerNpcTeleport(PlayerNpcTeleport::from_body(&mut cursor))
            }
            HealOtherPlayer::TAG => {
                Packet::HealOtherPlayer(HealOtherPlayer::from_body(&mut cursor))
            }
            Placeholder::TAG => Packet::Placeholder(Placeholder::from_body(&mut cursor)),
            ClientUuid::TAG => Packet::ClientUuid(ClientUuid::from_body(&mut cursor)),
            GetChestName::TAG => Packet::GetChestName(GetChestName::from_body(&mut cursor)),
            CatchNpc::TAG => Packet::CatchNpc(CatchNpc::from_body(&mut cursor)),
            ReleaseNpc::TAG => Packet::ReleaseNpc(ReleaseNpc::from_body(&mut cursor)),
            TravellingMerchantInventory::TAG => Packet::TravellingMerchantInventory(
                TravellingMerchantInventory::from_body(&mut cursor),
            ),
            TeleportationPotion::TAG => {
                Packet::TeleportationPotion(TeleportationPotion::from_body(&mut cursor))
            }
            AnglerQuest::TAG => Packet::AnglerQuest(AnglerQuest::from_body(&mut cursor)),
            CompleteAnglerQuest::TAG => {
                Packet::CompleteAnglerQuest(CompleteAnglerQuest::from_body(&mut cursor))
            }
            AnglerQuests::TAG => Packet::AnglerQuests(AnglerQuests::from_body(&mut cursor)),
            CreateTemporaryAnimation::TAG => {
                Packet::CreateTemporaryAnimation(CreateTemporaryAnimation::from_body(&mut cursor))
            }
            InvasionProgress::TAG => {
                Packet::InvasionProgress(InvasionProgress::from_body(&mut cursor))
            }
            PlaceObject::TAG => Packet::PlaceObject(PlaceObject::from_body(&mut cursor)),
            SyncPlayerChestIndex::TAG => {
                Packet::SyncPlayerChestIndex(SyncPlayerChestIndex::from_body(&mut cursor))
            }
            CreateCombatText::TAG => {
                Packet::CreateCombatText(CreateCombatText::from_body(&mut cursor))
            }
            LoadNetModule::TAG => Packet::LoadNetModule(LoadNetModule::from_body(&mut cursor)),
            SetNpcKillCount::TAG => {
                Packet::SetNpcKillCount(SetNpcKillCount::from_body(&mut cursor))
            }
            SetPlayerStealth::TAG => {
                Packet::SetPlayerStealth(SetPlayerStealth::from_body(&mut cursor))
            }
            QuickStash::TAG => Packet::QuickStash(QuickStash::from_body(&mut cursor)),
            UpdateTileEntity::TAG => {
                Packet::UpdateTileEntity(UpdateTileEntity::from_body(&mut cursor))
            }
            PlaceTileEntity::TAG => {
                Packet::PlaceTileEntity(PlaceTileEntity::from_body(&mut cursor))
            }
            TweakItem::TAG => Packet::TweakItem(TweakItem::from_body(&mut cursor)),
            PlaceItemFrame::TAG => Packet::PlaceItemFrame(PlaceItemFrame::from_body(&mut cursor)),
            UpdateItemDrop2::TAG => {
                Packet::UpdateItemDrop2(UpdateItemDrop2::from_body(&mut cursor))
            }
            SyncEmoteBubble::TAG => {
                Packet::SyncEmoteBubble(SyncEmoteBubble::from_body(&mut cursor))
            }
            SyncExtraValue::TAG => Packet::SyncExtraValue(SyncExtraValue::from_body(&mut cursor)),
            SocialHandshake::TAG => {
                Packet::SocialHandshake(SocialHandshake::from_body(&mut cursor))
            }
            94 => panic!("deprecated"),
            KillPortal::TAG => Packet::KillPortal(KillPortal::from_body(&mut cursor)),
            PlayerTeleportPortal::TAG => {
                Packet::PlayerTeleportPortal(PlayerTeleportPortal::from_body(&mut cursor))
            }
            PlayerNpcKilled::TAG => {
                Packet::PlayerNpcKilled(PlayerNpcKilled::from_body(&mut cursor))
            }
            SetEvent::TAG => Packet::SetEvent(SetEvent::from_body(&mut cursor)),
            UpdateMinionTarget::TAG => {
                Packet::UpdateMinionTarget(UpdateMinionTarget::from_body(&mut cursor))
            }
            NpcTeleportPortal::TAG => {
                Packet::NpcTeleportPortal(NpcTeleportPortal::from_body(&mut cursor))
            }
            UpdateShieldStrengths::TAG => {
                Packet::UpdateShieldStrengths(UpdateShieldStrengths::from_body(&mut cursor))
            }
            NebulaLevelUp::TAG => Packet::NebulaLevelUp(NebulaLevelUp::from_body(&mut cursor)),
            MoonLordCountdown::TAG => {
                Packet::MoonLordCountdown(MoonLordCountdown::from_body(&mut cursor))
            }
            NpcShopItem::TAG => Packet::NpcShopItem(NpcShopItem::from_body(&mut cursor)),
            GemLockToggle::TAG => Packet::GemLockToggle(GemLockToggle::from_body(&mut cursor)),
            PoofOfSmoke::TAG => Packet::PoofOfSmoke(PoofOfSmoke::from_body(&mut cursor)),
            SmartTextMessage::TAG => {
                Packet::SmartTextMessage(SmartTextMessage::from_body(&mut cursor))
            }
            WiredCannonShot::TAG => {
                Packet::WiredCannonShot(WiredCannonShot::from_body(&mut cursor))
            }
            MassWire::TAG => Packet::MassWire(MassWire::from_body(&mut cursor)),
            MassConsumeWire::TAG => {
                Packet::MassConsumeWire(MassConsumeWire::from_body(&mut cursor))
            }
            ToggleBirthdayParty::TAG => {
                Packet::ToggleBirthdayParty(ToggleBirthdayParty::from_body(&mut cursor))
            }
            GrowFx::TAG => Packet::GrowFx(GrowFx::from_body(&mut cursor)),
            CrystalInvasionStart::TAG => {
                Packet::CrystalInvasionStart(CrystalInvasionStart::from_body(&mut cursor))
            }
            CrystalInvasionWipe::TAG => {
                Packet::CrystalInvasionWipe(CrystalInvasionWipe::from_body(&mut cursor))
            }
            SetMinionTarget::TAG => {
                Packet::SetMinionTarget(SetMinionTarget::from_body(&mut cursor))
            }
            CrystalInvasionWait::TAG => {
                Packet::CrystalInvasionWait(CrystalInvasionWait::from_body(&mut cursor))
            }
            PlayerHurt::TAG => Packet::PlayerHurt(PlayerHurt::from_body(&mut cursor)),
            PlayerDeath::TAG => Packet::PlayerDeath(PlayerDeath::from_body(&mut cursor)),
            CombatText::TAG => Packet::CombatText(CombatText::from_body(&mut cursor)),
            Emoji::TAG => Packet::Emoji(Emoji::from_body(&mut cursor)),
            DollSync::TAG => Packet::DollSync(DollSync::from_body(&mut cursor)),
            InteractTileEntity::TAG => {
                Packet::InteractTileEntity(InteractTileEntity::from_body(&mut cursor))
            }
            PlaceWeaponRack::TAG => {
                Packet::PlaceWeaponRack(PlaceWeaponRack::from_body(&mut cursor))
            }
            HatRackSync::TAG => Packet::HatRackSync(HatRackSync::from_body(&mut cursor)),
            SyncTilePicking::TAG => {
                Packet::SyncTilePicking(SyncTilePicking::from_body(&mut cursor))
            }
            SyncRevenge::TAG => Packet::SyncRevenge(SyncRevenge::from_body(&mut cursor)),
            RemoveRevenge::TAG => Packet::RemoveRevenge(RemoveRevenge::from_body(&mut cursor)),
            LandGolfBall::TAG => Packet::LandGolfBall(LandGolfBall::from_body(&mut cursor)),
            ConnectionComplete::TAG => {
                Packet::ConnectionComplete(ConnectionComplete::from_body(&mut cursor))
            }
            FishOutNpc::TAG => Packet::FishOutNpc(FishOutNpc::from_body(&mut cursor)),
            TamperWithNpc::TAG => Packet::TamperWithNpc(TamperWithNpc::from_body(&mut cursor)),
            PlayLegacySound::TAG => {
                Packet::PlayLegacySound(PlayLegacySound::from_body(&mut cursor))
            }
            PlaceFood::TAG => Packet::PlaceFood(PlaceFood::from_body(&mut cursor)),
            UpdatePlayerLuck::TAG => {
                Packet::UpdatePlayerLuck(UpdatePlayerLuck::from_body(&mut cursor))
            }
            DeadPlayer::TAG => Packet::DeadPlayer(DeadPlayer::from_body(&mut cursor)),
            SyncMonsterType::TAG => {
                Packet::SyncMonsterType(SyncMonsterType::from_body(&mut cursor))
            }
            RequestNpcDebuff::TAG => {
                Packet::RequestNpcDebuff(RequestNpcDebuff::from_body(&mut cursor))
            }
            ClientSyncedInventory::TAG => {
                Packet::ClientSyncedInventory(ClientSyncedInventory::from_body(&mut cursor))
            }
            SetAsHost::TAG => Packet::SetAsHost(SetAsHost::from_body(&mut cursor)),

            tag => panic!(format!("unknown tag {}", tag)),
        }
    }
}
