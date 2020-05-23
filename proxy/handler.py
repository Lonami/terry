import struct
from collections import defaultdict

seen = defaultdict(int)

def handle(packet):
    if len(packet) < 4:
        # once: 06
        # mostly: 8a
        print(f'! len {len(packet)}?: {packet.hex()}')
        return

    tag = packet[2]
    _player_number = packet[3]
    body = packet[4:]
    seen[tag] += 1
