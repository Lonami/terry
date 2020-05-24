"""
Helper to generate the skeleton of new network packets.

Usage:
    python generatepackets.py <packet numbers...>
"""
from pathlib import Path

PACKETS_FOLDER = Path('src/packets')
MOD_UPDATES_FILE = Path('packets-mod.rs')

with open(MOD_UPDATES_FILE, 'w', encoding='utf-8') as mod_fd:
    for n in range(1, 140):
        mod_fd.write(f'''mod packet{n};
pub use packet{n}::Packet{n};
''')
        with open(PACKETS_FOLDER / f'packet{n}.rs', 'w', encoding='utf-8') as pk_fd:
            pk_fd.write(f'''use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// Packet {n}. No information known yet.
#[derive(Debug)]
pub struct Packet{n} {{
}}

impl PacketBody for Packet{n} {{
    const TAG: u8 = {n};

    fn write_body(&self, cursor: &mut SliceCursor) {{
        todo!()
    }}

    fn from_body(cursor: &mut SliceCursor) -> Self {{
        // TODO
        Self {{}}
    }}
}}
''')
