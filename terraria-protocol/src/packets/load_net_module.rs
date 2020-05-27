use crate::packets::PacketBody;
use crate::structures::{NetString, Vec2, RGB};
use crate::SliceCursor;

// prefix a byte with type: 0, 1, 2
#[derive(Debug)]
pub enum Bestiary {
    KillCount { npc_net_id: i16, kill_count: u16 },
    Sight { npc_net_id: i16 },
    Chat { npc_net_id: i16 },
}

/// Load a network module.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub enum LoadNetModule {
    Liquid {
        // TODO
    },
    // Client message is (command ID (text?), text)
    // Server message is (author ID, net text, RGB)
    // TODO ClientText and ServerText are the same packet but unfortunately
    //      the only way to differentiate them is to know "who we are", that
    //      is, if we were compiled as the client we would send ClientText
    //      and receive ServerText, and vice versa.
    //
    //      Unfortunately, we currently pretend to always be the client (by
    //      always expecting to receive from server), which means it would
    //      not work if we were actually being used as a client.
    //
    //      We can probably determine if there's RGB at the end of the message
    //      to determine who we are…
    ClientText {
        command: String,
        text: String,
    },
    ServerText {
        author: u8,
        text: NetString,
        color: RGB,
    },
    Text {
        command: String,
        text: String,
    },
    Ping {
        pos: Vec2,
    },
    Ambience {
        player: u8,
        num: i32,
        ty: u8,
    },
    Bestiary(Bestiary),
    CreativeUnlocks {
        item_id: i16,
        sacrifice_count: u16,
    },
    CreativePowers {
        power_id: u16,
    },
    CreativeUnlocksPlayerReport {
        zero: u8,
        item_id: u16,
        amount: u16,
    },
    TeleportPylon {
        /// If 2, it's a use request. Otherwise, added or removed.
        ty: u8,
        x: i16,
        y: i16,
        pylon_type: u8,
    },
    Particles {
        ty: u8,
        pos: Vec2,
        vel: Vec2,
        packed_shader_index: i32,
        /// Index of player who invoked this.
        player: u8,
    },
    CreativePowerPermissions {
        zero: u8,
        power_id: u16,
        level: u8,
    },
}

impl PacketBody for LoadNetModule {
    const TAG: u8 = 82;

    fn write_body(&self, cursor: &mut SliceCursor) {
        todo!()
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        match cursor.read::<u16>() {
            0 => LoadNetModule::Liquid {
                // TODO
            },
            1 => dbg!(LoadNetModule::ServerText {
                author: cursor.read(),
                text: cursor.read(),
                color: cursor.read(),
            }),
            2 => LoadNetModule::Ping { pos: cursor.read() },
            3 => LoadNetModule::Ambience {
                player: cursor.read(),
                num: cursor.read(),
                ty: cursor.read(),
            },
            4 => match cursor.read::<u8>() {
                0 => LoadNetModule::Bestiary(Bestiary::KillCount {
                    npc_net_id: cursor.read(),
                    kill_count: cursor.read(),
                }),
                1 => LoadNetModule::Bestiary(Bestiary::Sight {
                    npc_net_id: cursor.read(),
                }),
                2 => LoadNetModule::Bestiary(Bestiary::Chat {
                    npc_net_id: cursor.read(),
                }),
                n => panic!(format!("unknown bestiary net module {}", n)),
            },
            5 => LoadNetModule::CreativeUnlocks {
                item_id: cursor.read(),
                sacrifice_count: cursor.read(),
            },
            6 => LoadNetModule::CreativePowers {
                power_id: cursor.read(),
            },
            7 => LoadNetModule::CreativeUnlocksPlayerReport {
                zero: cursor.read(),
                item_id: cursor.read(),
                amount: cursor.read(),
            },
            8 => LoadNetModule::TeleportPylon {
                ty: cursor.read(),
                x: cursor.read(),
                y: cursor.read(),
                pylon_type: cursor.read(),
            },
            9 => LoadNetModule::Particles {
                ty: cursor.read(),
                pos: cursor.read(),
                vel: cursor.read(),
                packed_shader_index: cursor.read(),
                player: cursor.read(),
            },
            10 => LoadNetModule::CreativePowerPermissions {
                zero: cursor.read(),
                power_id: cursor.read(),
                level: cursor.read(),
            },
            n => panic!(format!("unknown net module {}", n)),
        }
    }
}
