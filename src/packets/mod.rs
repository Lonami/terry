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

use crate::serialization::{Deserializable, Serializable, SliceCursor};
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

impl Packet {
    pub fn from_slice(slice: &mut [u8]) -> Self {
        let mut cursor = SliceCursor::new(slice);
        let tag = cursor.read::<u8>();
        // TODO too bad packet body is not serializable
        match tag {
            ConnectRequest::TAG => Packet::ConnectRequest(ConnectRequest::from_body(&mut cursor)),
            Disconnect::TAG => Packet::Disconnect(Disconnect::from_body(&mut cursor)),
            SetUserSlot::TAG => Packet::SetUserSlot(SetUserSlot::from_body(&mut cursor)),
            PlayerInfo::TAG => Packet::PlayerInfo(PlayerInfo::from_body(&mut cursor)),
            PlayerInventorySlot::TAG => Packet::PlayerInventorySlot(PlayerInventorySlot::from_body(&mut cursor)),
            RequestWorldData::TAG => Packet::RequestWorldData(RequestWorldData::from_body(&mut cursor)),
            WorldInfo::TAG => Packet::WorldInfo(WorldInfo::from_body(&mut cursor)),
            RequestEssentialTiles::TAG => Packet::RequestEssentialTiles(RequestEssentialTiles::from_body(&mut cursor)),
            Status::TAG => Packet::Status(Status::from_body(&mut cursor)),
            SendSection::TAG => Packet::SendSection(SendSection::from_body(&mut cursor)),
            SectionTileFrame::TAG => Packet::SectionTileFrame(SectionTileFrame::from_body(&mut cursor)),
            SpawnPlayer::TAG => Packet::SpawnPlayer(SpawnPlayer::from_body(&mut cursor)),
            UpdatePlayer::TAG => Packet::UpdatePlayer(UpdatePlayer::from_body(&mut cursor)),
            PlayerActive::TAG => Packet::PlayerActive(PlayerActive::from_body(&mut cursor)),
            PlayerHP::TAG => Packet::PlayerHP(PlayerHP::from_body(&mut cursor)),
            ModifyTile::TAG => Packet::ModifyTile(ModifyTile::from_body(&mut cursor)),
            Time::TAG => Packet::Time(Time::from_body(&mut cursor)),
            DoorToggle::TAG => Packet::DoorToggle(DoorToggle::from_body(&mut cursor)),
            SendTileSquare::TAG => Packet::SendTileSquare(SendTileSquare::from_body(&mut cursor)),
            21 => panic!("tag 21 was replaced by tag 90"),
            UpdateItemOwner::TAG => Packet::UpdateItemOwner(UpdateItemOwner::from_body(&mut cursor)),
            NpcUpdate::TAG => Packet::NpcUpdate(NpcUpdate::from_body(&mut cursor)),
            StrikeNpc::TAG => Packet::StrikeNpc(StrikeNpc::from_body(&mut cursor)),
            25 => panic!("tag 25 is the null packet"),
            26 => panic!("tag 26 is the null packet"),
            ProjectileUpdate::TAG => Packet::ProjectileUpdate(ProjectileUpdate::from_body(&mut cursor)),
            NpcStrike::TAG => Packet::NpcStrike(NpcStrike::from_body(&mut cursor)),
            DestroyProjectile::TAG => Packet::DestroyProjectile(DestroyProjectile::from_body(&mut cursor)),
            TogglePVP::TAG => Packet::TogglePVP(TogglePVP::from_body(&mut cursor)),
            OpenChest::TAG => Packet::OpenChest(OpenChest::from_body(&mut cursor)),
            UpdateChestItem::TAG => Packet::UpdateChestItem(UpdateChestItem::from_body(&mut cursor)),
            SyncActiveChest::TAG => Packet::SyncActiveChest(SyncActiveChest::from_body(&mut cursor)),
            PlaceChest::TAG => Packet::PlaceChest(PlaceChest::from_body(&mut cursor)),
            HealEffect::TAG => Packet::HealEffect(HealEffect::from_body(&mut cursor)),
            PlayerZone::TAG => Packet::PlayerZone(PlayerZone::from_body(&mut cursor)),
            RequestPassword::TAG => Packet::RequestPassword(RequestPassword::from_body(&mut cursor)),
            SendPassword::TAG => Packet::SendPassword(SendPassword::from_body(&mut cursor)),
            RemoveItemOwner::TAG => Packet::RemoveItemOwner(RemoveItemOwner::from_body(&mut cursor)),
            SetActiveNpc::TAG => Packet::SetActiveNpc(SetActiveNpc::from_body(&mut cursor)),
            PlayerItemAnimation::TAG => Packet::PlayerItemAnimation(PlayerItemAnimation::from_body(&mut cursor)),
            PlayerMana::TAG => Packet::PlayerMana(PlayerMana::from_body(&mut cursor)),
            ManaEffect::TAG => Packet::ManaEffect(ManaEffect::from_body(&mut cursor)),
            44 => panic!("tag 44 is the null packet"),
            PlayerTeam::TAG => Packet::PlayerTeam(PlayerTeam::from_body(&mut cursor)),
            RequestSign::TAG => Packet::RequestSign(RequestSign::from_body(&mut cursor)),
            UpdateSign::TAG => Packet::UpdateSign(UpdateSign::from_body(&mut cursor)),
            SetLiquid::TAG => Packet::SetLiquid(SetLiquid::from_body(&mut cursor)),
            CompleteConnectionAndSpawn::TAG => Packet::CompleteConnectionAndSpawn(CompleteConnectionAndSpawn::from_body(&mut cursor)),
            UpdatePlayerBuff::TAG => Packet::UpdatePlayerBuff(UpdatePlayerBuff::from_body(&mut cursor)),
            SpecialNpcEffect::TAG => Packet::SpecialNpcEffect(SpecialNpcEffect::from_body(&mut cursor)),
            Unlock::TAG => Packet::Unlock(Unlock::from_body(&mut cursor)),
            AddNpcBuff::TAG => Packet::AddNpcBuff(AddNpcBuff::from_body(&mut cursor)),
            UpdateNpcBuff::TAG => Packet::UpdateNpcBuff(UpdateNpcBuff::from_body(&mut cursor)),
            AddPlayerBuff::TAG => Packet::AddPlayerBuff(AddPlayerBuff::from_body(&mut cursor)),
            UpdateNpcName::TAG => Packet::UpdateNpcName(UpdateNpcName::from_body(&mut cursor)),
            UpdateGoodEvil::TAG => Packet::UpdateGoodEvil(UpdateGoodEvil::from_body(&mut cursor)),
            PlayMusicItem::TAG => Packet::PlayMusicItem(PlayMusicItem::from_body(&mut cursor)),
            HitSwitch::TAG => Packet::HitSwitch(HitSwitch::from_body(&mut cursor)),
            SetNpcHome::TAG => Packet::SetNpcHome(SetNpcHome::from_body(&mut cursor)),
            SpawnBossInvasion::TAG => Packet::SpawnBossInvasion(SpawnBossInvasion::from_body(&mut cursor)),
            PlayerDodge::TAG => Packet::PlayerDodge(PlayerDodge::from_body(&mut cursor)),
            PaintTile::TAG => Packet::PaintTile(PaintTile::from_body(&mut cursor)),
            PaintWall::TAG => Packet::PaintWall(PaintWall::from_body(&mut cursor)),
            PlayerNpcTeleport::TAG => Packet::PlayerNpcTeleport(PlayerNpcTeleport::from_body(&mut cursor)),
            HealOtherPlayer::TAG => Packet::HealOtherPlayer(HealOtherPlayer::from_body(&mut cursor)),
            Placeholder::TAG => Packet::Placeholder(Placeholder::from_body(&mut cursor)),
            ClientUUID::TAG => Packet::ClientUUID(ClientUUID::from_body(&mut cursor)),
            GetChestName::TAG => Packet::GetChestName(GetChestName::from_body(&mut cursor)),
            CatchNpc::TAG => Packet::CatchNpc(CatchNpc::from_body(&mut cursor)),
            ReleaseNpc::TAG => Packet::ReleaseNpc(ReleaseNpc::from_body(&mut cursor)),
            TravellingMerchantInventory::TAG => Packet::TravellingMerchantInventory(TravellingMerchantInventory::from_body(&mut cursor)),
            TeleportationPotion::TAG => Packet::TeleportationPotion(TeleportationPotion::from_body(&mut cursor)),
            AnglerQuest::TAG => Packet::AnglerQuest(AnglerQuest::from_body(&mut cursor)),
            CompleteAnglerQuest::TAG => Packet::CompleteAnglerQuest(CompleteAnglerQuest::from_body(&mut cursor)),
            AnglerQuests::TAG => Packet::AnglerQuests(AnglerQuests::from_body(&mut cursor)),
            CreateTemporaryAnimation::TAG => Packet::CreateTemporaryAnimation(CreateTemporaryAnimation::from_body(&mut cursor)),
            InvasionProgress::TAG => Packet::InvasionProgress(InvasionProgress::from_body(&mut cursor)),
            PlaceObject::TAG => Packet::PlaceObject(PlaceObject::from_body(&mut cursor)),
            SyncPlayerChestIndex::TAG => Packet::SyncPlayerChestIndex(SyncPlayerChestIndex::from_body(&mut cursor)),
            CreateCombatText::TAG => Packet::CreateCombatText(CreateCombatText::from_body(&mut cursor)),
            LoadNetModule::TAG => Packet::LoadNetModule(LoadNetModule::from_body(&mut cursor)),
            SetNpcKillCount::TAG => Packet::SetNpcKillCount(SetNpcKillCount::from_body(&mut cursor)),
            SetPlayerStealth::TAG => Packet::SetPlayerStealth(SetPlayerStealth::from_body(&mut cursor)),
            QuickStash::TAG => Packet::QuickStash(QuickStash::from_body(&mut cursor)),
            UpdateTileEntity::TAG => Packet::UpdateTileEntity(UpdateTileEntity::from_body(&mut cursor)),
            PlaceTileEntity::TAG => Packet::PlaceTileEntity(PlaceTileEntity::from_body(&mut cursor)),
            TweakItem::TAG => Packet::TweakItem(TweakItem::from_body(&mut cursor)),
            PlaceItemFrame::TAG => Packet::PlaceItemFrame(PlaceItemFrame::from_body(&mut cursor)),
            UpdateItemDrop2::TAG => Packet::UpdateItemDrop2(UpdateItemDrop2::from_body(&mut cursor)),
            SyncEmoteBubble::TAG => Packet::SyncEmoteBubble(SyncEmoteBubble::from_body(&mut cursor)),
            SyncExtraValue::TAG => Packet::SyncExtraValue(SyncExtraValue::from_body(&mut cursor)),
            SocialHandshake::TAG => Packet::SocialHandshake(SocialHandshake::from_body(&mut cursor)),
            94 => panic!("deprecated"),
            KillPortal::TAG => Packet::KillPortal(KillPortal::from_body(&mut cursor)),
            PlayerTeleportPortal::TAG => Packet::PlayerTeleportPortal(PlayerTeleportPortal::from_body(&mut cursor)),
            PlayerNpcKilled::TAG => Packet::PlayerNpcKilled(PlayerNpcKilled::from_body(&mut cursor)),
            SetEvent::TAG => Packet::SetEvent(SetEvent::from_body(&mut cursor)),
            UpdateMinionTarget::TAG => Packet::UpdateMinionTarget(UpdateMinionTarget::from_body(&mut cursor)),
            NpcTeleportPortal::TAG => Packet::NpcTeleportPortal(NpcTeleportPortal::from_body(&mut cursor)),
            UpdateShieldStrengths::TAG => Packet::UpdateShieldStrengths(UpdateShieldStrengths::from_body(&mut cursor)),
            NebulaLevelUp::TAG => Packet::NebulaLevelUp(NebulaLevelUp::from_body(&mut cursor)),
            MoonLordCountdown::TAG => Packet::MoonLordCountdown(MoonLordCountdown::from_body(&mut cursor)),
            NpcShopItem::TAG => Packet::NpcShopItem(NpcShopItem::from_body(&mut cursor)),
            GemLockToggle::TAG => Packet::GemLockToggle(GemLockToggle::from_body(&mut cursor)),
            PoofofSmoke::TAG => Packet::PoofofSmoke(PoofofSmoke::from_body(&mut cursor)),
            SmartTextMessage::TAG => Packet::SmartTextMessage(SmartTextMessage::from_body(&mut cursor)),
            WiredCannonShot::TAG => Packet::WiredCannonShot(WiredCannonShot::from_body(&mut cursor)),
            MassWire::TAG => Packet::MassWire(MassWire::from_body(&mut cursor)),
            MassConsumeWire::TAG => Packet::MassConsumeWire(MassConsumeWire::from_body(&mut cursor)),
            ToggleBirthdayParty::TAG => Packet::ToggleBirthdayParty(ToggleBirthdayParty::from_body(&mut cursor)),
            GrowFx::TAG => Packet::GrowFx(GrowFx::from_body(&mut cursor)),
            CrystalInvasionStart::TAG => Packet::CrystalInvasionStart(CrystalInvasionStart::from_body(&mut cursor)),
            CrystalInvasionWipe::TAG => Packet::CrystalInvasionWipe(CrystalInvasionWipe::from_body(&mut cursor)),
            SetMinionTarget::TAG => Packet::SetMinionTarget(SetMinionTarget::from_body(&mut cursor)),
            CrystalInvasionWait::TAG => Packet::CrystalInvasionWait(CrystalInvasionWait::from_body(&mut cursor)),
            PlayerHurt::TAG => Packet::PlayerHurt(PlayerHurt::from_body(&mut cursor)),
            PlayerDeath::TAG => Packet::PlayerDeath(PlayerDeath::from_body(&mut cursor)),
            CombatText::TAG => Packet::CombatText(CombatText::from_body(&mut cursor)),
            Emoji::TAG => Packet::Emoji(Emoji::from_body(&mut cursor)),
            DollSync::TAG => Packet::DollSync(DollSync::from_body(&mut cursor)),
            InteractTileEntity::TAG => Packet::InteractTileEntity(InteractTileEntity::from_body(&mut cursor)),
            PlaceWeaponRack::TAG => Packet::PlaceWeaponRack(PlaceWeaponRack::from_body(&mut cursor)),
            HatRackSync::TAG => Packet::HatRackSync(HatRackSync::from_body(&mut cursor)),
            SyncTilePicking::TAG => Packet::SyncTilePicking(SyncTilePicking::from_body(&mut cursor)),
            SyncRevenge::TAG => Packet::SyncRevenge(SyncRevenge::from_body(&mut cursor)),
            RemoveRevenge::TAG => Packet::RemoveRevenge(RemoveRevenge::from_body(&mut cursor)),
            LandGolfBall::TAG => Packet::LandGolfBall(LandGolfBall::from_body(&mut cursor)),
            ConnectionComplete::TAG => Packet::ConnectionComplete(ConnectionComplete::from_body(&mut cursor)),
            FishOutNpc::TAG => Packet::FishOutNpc(FishOutNpc::from_body(&mut cursor)),
            TamperWithNpc::TAG => Packet::TamperWithNpc(TamperWithNpc::from_body(&mut cursor)),
            PlayLegacySound::TAG => Packet::PlayLegacySound(PlayLegacySound::from_body(&mut cursor)),
            PlaceFood::TAG => Packet::PlaceFood(PlaceFood::from_body(&mut cursor)),
            UpdatePlayerLuck::TAG => Packet::UpdatePlayerLuck(UpdatePlayerLuck::from_body(&mut cursor)),
            DeadPlayer::TAG => Packet::DeadPlayer(DeadPlayer::from_body(&mut cursor)),
            SyncMonsterType::TAG => Packet::SyncMonsterType(SyncMonsterType::from_body(&mut cursor)),
            RequestNpcDebuff::TAG => Packet::RequestNpcDebuff(RequestNpcDebuff::from_body(&mut cursor)),
            ClientSyncedInventory::TAG => Packet::ClientSyncedInventory(ClientSyncedInventory::from_body(&mut cursor)),
            SetAsHost::TAG => Packet::SetAsHost(SetAsHost::from_body(&mut cursor)),

            tag => panic!(format!("unknown tag {}", tag)),
        }
    }
}

#[derive(Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new() -> Self {
        RGB { r: 0, g: 0, b: 0 }
    }
}

impl Serializable for RGB {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.r);
        cursor.write(&self.g);
        cursor.write(&self.b);
    }
}

impl Deserializable for RGB {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self {
            r: cursor.read(),
            g: cursor.read(),
            b: cursor.read(),
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    pub flags: [u8; 2],
    pub color: Option<u8>,
    pub wall_color: Option<u8>,
    pub ty: Option<u16>,
    pub frame_x: Option<u16>,
    pub frame_y: Option<u16>,
    pub wall: Option<u16>,
    pub liquid: Option<u8>,
    pub liquid_type: Option<u8>,
}

impl Tile {
    fn active(&self) -> bool { self.flags[0] & 0x01 != 0 }
    fn lighted(&self) -> bool { self.flags[0] & 0x02 != 0 }
    fn has_wall(&self) -> bool { self.flags[0] & 0x04 != 0 }
    fn has_liquid(&self) -> bool { self.flags[0] & 0x08 != 0 }
    fn wire1(&self) -> bool { self.flags[0] & 0x10 != 0 }
    fn half_brick(&self) -> bool { self.flags[0] & 0x20 != 0 }
    fn actuator(&self) -> bool { self.flags[0] & 0x40 != 0 }
    fn inactive(&self) -> bool { self.flags[0] & 0x80 != 0 }

    fn wire2(&self) -> bool { self.flags[1] & 0x01 != 0 }
    fn wire3(&self) -> bool { self.flags[1] & 0x02 != 0 }
    fn has_color(&self) -> bool { self.flags[1] & 0x04 != 0 }
    fn has_wall_color(&self) -> bool { self.flags[1] & 0x08 != 0 }
    fn slope1(&self) -> bool { self.flags[1] & 0x10 != 0 }
    fn slope2(&self) -> bool { self.flags[1] & 0x20 != 0 }
    fn slope3(&self) -> bool { self.flags[1] & 0x40 != 0 }
    fn wire4(&self) -> bool { self.flags[1] & 0x80 != 0 }
}

impl Serializable for Tile {
    fn serialize(&self, cursor: &mut SliceCursor) {
        // TODO yes we need a better way to deal with bitflags and options
        self.flags.iter().for_each(|f| cursor.write(f));
        if self.has_color() {
            cursor.write(&self.color.unwrap());
        }
        if self.has_wall_color() {
            cursor.write(&self.wall_color.unwrap());
        }
        if self.active() {
            cursor.write(&self.ty.unwrap());
        }
        let tile_frame_important = true; // ???
        if self.active() && tile_frame_important {
            cursor.write(&self.frame_x.unwrap());
            cursor.write(&self.frame_y.unwrap());
        }
        if self.has_wall() {
            cursor.write(&self.wall.unwrap());
        }
        if self.has_liquid() {
            cursor.write(&self.liquid.unwrap());
            cursor.write(&self.liquid_type.unwrap());
        }
    }
}

impl Deserializable for Tile {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct Chest {
    pub index: u16,
    pub x: u16,
    pub y: u16,
    pub name: String,
}

impl Serializable for Chest {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.index);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.name);
    }
}

impl Deserializable for Chest {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self {
            index: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            name: cursor.read(),
        }
    }
}

#[derive(Debug)]
pub struct Sign {
    pub index: u16,
    pub x: u16,
    pub y: u16,
    pub text: String,
}

impl Serializable for Sign {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.index);
        cursor.write(&self.x);
        cursor.write(&self.y);
        cursor.write(&self.text);
    }
}

impl Deserializable for Sign {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self {
            index: cursor.read(),
            x: cursor.read(),
            y: cursor.read(),
            text: cursor.read(),
        }
    }
}

enum TileEntity {
    TrainingDummy {
        id: i32,
        x: u16,
        y: u16,
        npc_index: u16,
    },
    ItemFrame {
        id: i32,
        x: u16,
        y: u16,
        item_type: u16,
        item_prefix: u8,
        item_stack: u16,
    },
    LogicSensor {
        id: i32,
        x: u16,
        y: u16,
        logic_check_type: u8,
        on: bool,
    },
    DisplayDoll {
        id: i32,
        x: u16,
        y: u16,
        flags: [u8; 2], // TODO read body
    },
    WeaponRack {
        id: i32,
        x: u16,
        y: u16,
        item_type: u16,
        item_prefix: u8,
        item_stack: u16,
    },
    HatRack {
        id: i32,
        x: u16,
        y: u16,
        flags: u8, // TODO read body
    },
    FloodPlatter {
        id: i32,
        x: u16,
        y: u16,
        item_type: u16,
        item_prefix: u8,
        item_stack: u16,
    },
    Pylon {
        id: i32,
        x: u16,
        y: u16,
    },
}

impl Serializable for TileEntity {
    fn serialize(&self, cursor: &mut SliceCursor) {
        todo!()
    }
}

impl Deserializable for TileEntity {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        todo!()
    }
}


#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NetStringMode {
    Literal = 0,
    Formattable = 1,
    LocalizationKey = 2,
}

impl Serializable for NetStringMode {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&(*self as u8));
    }
}

impl Deserializable for NetStringMode {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        match cursor.read::<u8>() {
            0 => NetStringMode::Literal,
            1 => NetStringMode::Formattable,
            2 => NetStringMode::LocalizationKey,
            n => panic!(format!("invalid net string mode {}", n)),
        }
    }
}

#[derive(Debug)]
pub struct NetString {
    mode: NetStringMode,
    text: String,
    substitutions: Vec<NetString>,
}

impl Serializable for NetString {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.mode);
        cursor.write(&self.text);
        if self.mode != NetStringMode::Literal {
            let len: u8 = self
                .substitutions
                .len()
                .try_into()
                .expect("too many substitutions");
            cursor.write(&len);
            self.substitutions.iter().for_each(|s| cursor.write(s));
        }
    }
}

impl Deserializable for NetString {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        let mode = cursor.read();
        let text = cursor.read();
        let mut substitutions = Vec::new();
        if mode != NetStringMode::Literal {
            let len = cursor.read::<u8>() as usize;
            substitutions.reserve(len);
            (0..len).for_each(|_| substitutions.push(cursor.read()));
        }
        Self {
            mode,
            text,
            substitutions,
        }
    }
}

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Serializable for Vec2 {
    fn serialize(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.x);
        cursor.write(&self.y);
    }
}

impl Deserializable for Vec2 {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        Self {
            x: cursor.read(),
            y: cursor.read(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_good_packet_serialization() {
        let mut buffer = vec![0; 64];
        let mut cursor = SliceCursor::new(buffer.as_mut_slice());
        Connect {
            magic: "Terraria228".to_string(),
        }
        .serialize(None, &mut cursor);
        let pos = cursor.finish();
        assert_eq!(
            &buffer[..pos],
            &[
                0x0f, 0x00, 0x01, 0x0b, 0x54, 0x65, 0x72, 0x72, 0x61, 0x72, 0x69, 0x61, 0x32, 0x32,
                0x38
            ]
        );
    }
}
