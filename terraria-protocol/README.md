# terraria-protocol

This library contains all the packet definitions needed to work with
Terraria's multiplayer network protocol, and it can be used to build
a client, a robot, or even a custom Terraria server.

Note that the library itself doesn't do anything interesting, only the basics.
To see what can be built, refer to [terry], the original motivation behind
this work.

Huge thanks to [tshock's documentation on the Multiplayer Packet Structure][ts-mps].
It has really helped to get most of the protocol without having to spend an
absurd amount of time guessing.

[terry]: https://github.com/Lonami/terry
[ts-mps]: https://tshock.readme.io/docs/multiplayer-packet-structure
