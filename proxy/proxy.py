import importlib
import os
import select
import socket
import struct
import time
import traceback

# module with hot-reloading
import handler

print('connecting...')
terraria = socket.socket()
while True:
    try:
        terraria.connect(('localhost', 7777))
        break
    except ConnectionRefusedError:
        time.sleep(1)

print('listening...')
server = socket.socket()
server.bind(('', 8888))
server.listen(1)
client, _ = server.accept()

buf = b''

print('running...')
try:
    last_mod = os.path.getmtime(handler.__file__)
    while True:
        ready, _, _ = select.select([client, terraria], [], [])
        for s in ready:
            chunk = s.recv(1024)
            if s == terraria:
                client.send(chunk)
            elif s == client:
                terraria.send(chunk)

                # analyze client packets, all prefixed with their length
                buf += chunk
                while buf and len(buf) >= struct.unpack('<H', buf[:2])[0]:
                    length = struct.unpack('<H', buf[:2])[0]
                    packet, buf = buf[:length], buf[length:]

                    try:
                        cur_mod = os.path.getmtime(handler.__file__)
                        if cur_mod != last_mod:
                            last_mod = cur_mod
                            importlib.reload(handler)
                            print('reloaded handler')

                        handler.handle(packet)
                    except Exception:
                        traceback.print_exc()
except BrokenPipeError:
    pass
finally:
    server.close()
    client.close()
    terraria.close()
