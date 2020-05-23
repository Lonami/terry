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

def handle_1(tag, body):
    print('protocol magic:', body.decode('ascii'))

def handle_4(tag, body):
    name_len = struct.unpack('<B', body[2:3])[0]
    name = body[3:3 + name_len].decode('ascii')
    print('player name:', name)

def handle_5(tag, body):
    pass  # spammed a lot

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
    assert c == 0b00000010
    assert d == 0b00000000

    # normalize
    nx = round((x - WORLD_CENTER_X) / FT_POS_SCALE, 2)
    ny = round((WORLD_SURFACE_Y - y) / FT_POS_SCALE, 2)

    if speed_flag & 0b0000_0100:
        dx, dy = struct.unpack('<ff', body[MOVEMENT.size:])
        dx = round(dx, 2)
        dy = round(dy, 2)
        print('item', hotbar, 'move', nx, ny, '( + speed', dx, dy, ')')
    else:
        print('item', hotbar, 'move', nx, ny)

def handle_50(tag, body):
    mount_id = body[6]
    print('using mount', mount_id)

def handle_68(tag, body):
    print('player uuid4:', body.decode('ascii'))

def handle_new(tag, body):
    print(f'new {tag} ({seen[tag]}): {body.hex()}')
    pass  # so we can comment out the print easily

def handle(packet):
    tag = packet[2]
    seen[tag] += 1

    if tag in (6, 138):
        return  # these don't have more info

    _player_index = packet[3]
    body = packet[4:]

    (globals().get(f'handle_{tag}') or handle_new)(tag, body)
