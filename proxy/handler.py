import struct
from collections import defaultdict

seen = defaultdict(int)

MOVEMENT = struct.Struct('<BBBBBff')

# small world
WORLD_START_X = 656
WORLD_CENTER_X = 33590
WORLD_END_X = 66508

WORLD_START_Y = 656
WORLD_SURFACE_Y = 5334
WORLD_END_Y = 18486
FT_POS_SCALE = 8

HELD_ITEM_INVENTORY_INDEX = 58

def handle_1(tag, body):
    print('protocol magic:', body.decode('ascii'))

def handle_4(tag, body):
    name_len = struct.unpack('<B', body[2:3])[0]
    name = body[3:3 + name_len].decode('ascii')
    print('player name:', name)

def handle_5(tag, body):
    # inventory
    index, count, a, item_id = struct.unpack('<HHBH', body)
    # a = 72 for accessories
    # a = 73 or 76 for vanity accessories
    # a = 0 otherwise
    if index == HELD_ITEM_INVENTORY_INDEX:
        print(f'holding {count} items with id {item_id}')
    else:
        print(f'inventory: {count} items with id {item_id} at pos {index}, a {a}')

def handle_12(tag, body):
    x, y, timer, how = struct.unpack('<HHIB', body)
    # x, y seem to always be ffff
    # how = 0 for death
    # how = 2 for recall or mirror
    print('back to spawn', x, y, timer, how)

def handle_13(tag, body):
    flags, speed_flag, c, d, hotbar, x, y = MOVEMENT.unpack(body[:MOVEMENT.size])
    if flags & 0b0000_0001:
        print('holding up')
    if flags & 0b0000_0010:
        print('holding down')
    if flags & 0b0000_0100:
        print('holding left')
    if flags & 0b0000_1000:
        print('holding right')
    if flags & 0b0001_0000:
        print('holding space')
    if flags & 0b0010_0000:
        print('holding mouse')
    if flags & 0b0100_0000:
        print('facing right')

    assert not (flags & 0b1000_0000)
    assert (speed_flag & 0b1111_1011) == 0b0001_0000
    assert c in (0, 0b00000010)  # seems 0 for void bag
    assert d == 0b00000000

    # normalize
    nx = round((x - WORLD_CENTER_X) / FT_POS_SCALE, 2)
    ny = round((WORLD_SURFACE_Y - y) / FT_POS_SCALE, 2)

    if speed_flag & 0b0000_0100:
        dx, dy = struct.unpack('<ff', body[MOVEMENT.size:])
        dx = round(dx, 2)
        dy = round(dy, 2)
    else:
        dx = 0.0
        dy = 0.0

    #print('item', hotbar, 'move', nx, ny, '( + speed', dx, dy, ')')

def handle_21(tag, body):
    # item moved in the world
    created, x, y, dx, dy, count, modifier, d, item_id = struct.unpack('<?ffffHBBH', body)
    assert d == 0

def handle_22(tag, body):
    a, b = struct.unpack('<BB', body)
    # seems to always be 00ff or ff00

def handle_50(tag, body):
    mount_id = body[6]
    print('using mount', mount_id)

def handle_68(tag, body):
    print('player uuid4:', body.decode('ascii'))

def handle_new(tag, body):
    #print(f'new {tag} ({seen[tag]}): {body.hex()}')
    pass  # so we can comment out the print easily

def handle(packet):
    tag = packet[2]
    seen[tag] += 1

    if tag in (6, 138):
        return  # these don't have more info

    _player_index = packet[3]
    body = packet[4:]

    (globals().get(f'handle_{tag}') or handle_new)(tag, body)
