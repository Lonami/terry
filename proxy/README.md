# terry proxy

This folder contains some Python code to quickly iterate in the analysis on
the network protocol used by the official Terraria client.

It's a very simple proxy which supports hot-reloading of the `handler.py`
module so you can add, break and change code while the proxy still runs.

## usage

Run `python proxy.py` while there's a Terraria 1.4 server running on default
host and port (localhost:7777) and edit `handler.py` on the fly to figure
out how things work.
