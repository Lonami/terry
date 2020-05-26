## History

At the beginning, a custom proxy was used to watch the network packets come
and go, while doing things with the client to see what packets they produce.

This approach gets tiring quickly, and if I had done a little more of
research earlier on, I would've found that tshock has published the
[Multiplayer Packet Structure][1] which has been incredibly useful to save
a lot of time.

However, another mistake was made, and it was manually writing the hundred
of packets that exist.

After a bad automation of the process, a lot of time was spent renaming and
reviewing everything by hand, because the generation was only for the
skeleton of the packets and very basic.

If I had to start again, I would automatically generating a custom DSL to
describe the packets, clean it up, and generate proper code with it.

[1]: https://tshock.readme.io/docs/multiplayer-packet-structure
