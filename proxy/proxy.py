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


def check_buf(chunk, *, remote, persisted=[b'', b'', os.path.getmtime(handler.__file__)]):
    buf = persisted[remote] + chunk

    # all packets are prefixed with their length
    while len(buf) >= 2 and len(buf) >= struct.unpack('<H', buf[:2])[0]:
        length = struct.unpack('<H', buf[:2])[0]
        packet, buf = buf[:length], buf[length:]

        try:
            mod = os.path.getmtime(handler.__file__)
            if mod != persisted[-1]:
                persisted[-1] = mod
                importlib.reload(handler)
                print('reloaded handler')

            handler.handle(remote, packet)
        except Exception:
            traceback.print_exc()

    persisted[remote] = buf


print('running...')
try:
    while True:
        ready, _, _ = select.select([client, terraria], [], [])
        for s in ready:
            chunk = s.recv(1024)
            if s == terraria:
                client.send(chunk)
                check_buf(chunk, remote=True)
            elif s == client:
                terraria.send(chunk)
                check_buf(chunk, remote=False)
except BrokenPipeError:
    pass
finally:
    server.close()
    client.close()
    terraria.close()
    handler.finish()
