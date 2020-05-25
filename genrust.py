import mistune
import re
import html
from bprint import bprint

mods = ''
pub_use = ''
enum = ''

def generate(tag, name, direction, fields):
    if direction == 'Never sent':
        print('never sent:', tag)
        return

    global mods, pub_use, enum

    struct_name = re.sub(r'\s+', '', name)
    file_name = re.sub(r'\s+', '_', name).lower()

    mods += f'mod {file_name};\n'
    pub_use += f'pub use {file_name}::{struct_name};\n'
    enum += f'    {struct_name}({struct_name}), // {tag}\n'

    with open(f'rust/{file_name}.rs', 'w', encoding='utf-8') as fd:
        # this indent is so confusing so use comments to not get lost
        # begin
        fd.write(f'''\
use crate::packets::PacketBody;
use crate::serialization::SliceCursor;

/// {name}.
///
/// Direction: {direction}.
#[derive(Debug)]
pub struct {struct_name} {{
''')
        # end

        for field in fields:
            if field['note']:
                fd.write('    /// ')
                fd.write(field['note'])
                fd.write('\n')
            fd.write('    pub ')
            fd.write(field['name'])
            fd.write(': ')
            fd.write(field['type'])
            fd.write(',\n')

        # begin
        fd.write(f'''\
}}

impl PacketBody for {struct_name} {{
    const TAG: u8 = {tag};

    fn write_body(&self, cursor: &mut SliceCursor) {{
''')
        # end

        for field in fields:
            fd.write('        cursor.write(&self.')
            fd.write(field['name'])
            fd.write(');\n')

        # begin
        fd.write(f'''\
    }}

    fn from_body(cursor: &mut SliceCursor) -> Self {{
        Self {{
''')
        # end

        for field in fields:
            fd.write('            ')
            fd.write(field['name'])
            fd.write(': cursor.read(),\n')

        # begin
        fd.write(f'''\
        }}
    }}
}}
''')
        # end

def sanitize_field_name(name):
    return re.sub(r'\s+', '_', name.strip()).lower()

def sanitize_field_type(ty):
    return {
        'String': 'String',
        'NetworkText': 'NetString',
        'Byte': 'u8',
        'Color': 'Color',
        'Int16': 'i16',
        'Int32': 'i32',
        'UInt64': 'u64',
        'Single': 'i32 /* single */ ',
        'SByte': 'i8',
        'Boolean': 'bool',
        '-': '()',
        'UInt16': 'u16',
        'Variable': '() /* variable */ ',
        'Unsigned Short': 'u16',
        'UInt32': 'u32',
        'Float': 'f32',
        'Bool': 'bool',
        'PlayerDeathReason': 'PlayerDeathReason',
    }[ty]

def sanitize_field_note(note):
    note = note.strip()
    if not note or note == '-':
        return None
    return note.replace('\n', ' ').strip()

def parse():
    parse = mistune.create_markdown(
        # escape=False,  # seems to work poorly anyway
        renderer='ast',
        plugins=['url', 'strikethrough', 'footnotes', 'table']
    )

    with open('mps.md', encoding='utf-8') as fd:
        markdown = parse(fd.read())

    tag = 0
    name = 'Unknown'
    direction = 'Unknown'
    fields = None

    for item in markdown:
        if item['type'] == 'heading' and item['level'] == 2:
            try:
                name = html.unescape(item['children'][0]['text'].strip())
                tag = int(item['children'][1]['text'].strip('[]'))
            except (IndexError, ValueError):
                tag = 0  # not interested in this heading
        elif item['type'] == 'heading' and item ['level'] == 3:
            direction = html.unescape(item['children'][0]['text'].strip())
        elif item['type'] == 'table' and tag != 0:
            fields = []
            for i, row in enumerate(item['children'][1]['children']):
                if len(row['children']) != 4:
                    print('bad row', tag, name, ':', i)
                    continue

                _size, desc, ty, notes = row['children']
                fields.append(dict(
                    name=sanitize_field_name(desc['children'][0]['text']),
                    type=sanitize_field_type(ty['children'][0]['text']),
                    note=sanitize_field_note(notes['children'][0]['text'] if notes['children'] else '-')
                ))

            # we're done
            generate(tag, name, direction, fields)

    with open('rust/mod.rs', 'w', encoding='utf-8') as fd:
        fd.write(mods)
        fd.write('\n')
        fd.write(pub_use)
        fd.write('\nenum Packet {\n')
        fd.write(enum)
        fd.write('}\n')

if __name__ == '__main__':
    parse()
