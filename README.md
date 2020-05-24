# terry

A player bot for Terraria 1.4 written in Rust.

## how?

By using a proxy and watching the network packets come and go, while
doing things with a real client to see what packets they produce.

This approach gets tiring quickly, and if I had done a little more of
research earlier on, I would've found that tshock has published the
[Multiplayer Packet Structure][1] which I will make use of to finish
off the rest of the packets.

[1]: https://tshock.readme.io/docs/multiplayer-packet-structure
