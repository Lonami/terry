import importlib
import os
import select
import socket
import struct
import time
import traceback
import sys

# module with hot-reloading
import handler


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


if '--offline' in sys.argv[1:]:
    print('running with offline data...')

    fds = [open('local.bin', 'rb'), open('remote.bin', 'rb')]
    try:
        check_buf(fds[0].read(), remote=False)
        check_buf(fds[1].read(), remote=True)
    finally:
        fds[0].close()
        fds[1].close()
else:
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

    print('running...')
    fds = [open('local.bin', 'wb'), open('remote.bin', 'wb')]
    try:
        while True:
            ready, _, _ = select.select([client, terraria], [], [])
            for s in ready:
                chunk = s.recv(1024)
                if s == terraria:
                    client.send(chunk)
                    remote = True
                elif s == client:
                    terraria.send(chunk)
                    remote = False
                fds[remote].write(chunk)
                check_buf(chunk, remote=remote)
    except BrokenPipeError:
        pass
    finally:
        server.close()
        client.close()
        terraria.close()
        handler.finish()
        fds[0].close()
        fds[1].close()
