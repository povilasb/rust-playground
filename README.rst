=====
About
=====

This is solution to Option B "Client/Server" problem.
It is written by Povilas - https://github.com/povilasb.

Usage
=====

Run the server with `cargo run` inside `server/` directory.
Use the client to send commands::

    $ cd client
    $ python client.py

Comments
========

I spent exactly 3 hours on the problem.
I didn't have enough time to build proper server with robust error handling, etc.
It's a barely working mio based implementation.
I didn't code client. Although, there's a small python script that helps
testing.
Anyway, it wouldn't be too hard to reuse `proto` module from the server and
use blocking networking API to build a Rust based client :)

Auto discovery
==============

To discover the servers I'd use something like bittorrent trackers.
Meaning I'd build a centralized server that keeps track of worker servers.
Each new server would register on the tracker, etc.

Alternatively, if I really wanted to be it fully decentralized, I could ship
my applications with some preinstalled list of available servers.
Then when new servers join the network, they could broadcast their appearance
and update the lists in running servers...

Requirements
============

* stable Rust (tested with `rustc 1.20.0 (f3d6973f4 2017-08-27)`).

Testing
=======

Run unit tests with::

    $ cargo test
