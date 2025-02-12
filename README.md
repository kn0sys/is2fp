[![cargo-build](https://github.com/kn0sys/is2fp/actions/workflows/rust.yml/badge.svg)](https://github.com/kn0sys/is2fp/actions/workflows/rust.yml)

# is2fp
Dandelion-IS2FP uses random i2p relay servers for stem selection and fluff propagation

## Invisible stem-to-fluff propagation

* mDNS stem discovery via ip address
* nodes execute i2p base 32 exchange
* random node selected for stem (invisible stem selection)
* proof-of-work solution published with fluff propagated message
* TODO: network consensus of pow

### Getting Started

`git clone --recursive https://github.com/kn0sys/is2fp`

### IS2FP Console

* Add peer manually `add peer <Multiaddr>`
* Send `send MESSAGE>` sends a message via chat

### API

* `/message` - recieve a message to propagate
* `/i2p/status` - check i2p status
* TODO: add peer, etc.

### j4-i2p-rs - embedded i2p

 * see [j4i2prs](https://github.com/kn0sys/j4-i2p-rs) for building jars
 * copy `certificates` and `opt` directories to `is2fp` root directory
 * for multiple nodes on one machine set the appropiate environment variables after the first node
    * `IS2FP_ROUTER_OVERRIDE=1`
    * `IS2FP_PORT=<PORT>`
    * `IS2FP_LMDB_ENV=<testX>`

### Sample Network Operation

Alice 

```bash
RUST_LOG=none,is2fp=debug cargo run
   Compiling is2fp v0.1.0-alpha (/home/user/is2fp)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.48s
     Running `target/debug/is2fp`
[2025-02-12T03:25:07Z INFO  is2fp::utils] dandelion-is2fp is starting up
[2025-02-12T03:25:07Z INFO  is2fp::db] setting lmdb map size to: 1252564224
[2025-02-12T03:25:07Z INFO  is2fp::db] $LMDB_USER=user
[2025-02-12T03:25:07Z INFO  is2fp::db] excecuting lmdb open
[2025-02-12T03:25:07Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:25:07Z WARN  is2fp::utils] failed to read i2p proxy host: NotPresent
[2025-02-12T03:25:07Z INFO  is2fp::i2p] starting j4i2prs...
[2025-02-12T03:25:07Z INFO  is2fp::utils] relay server address - 4cgh4c26jwyr6nuk4lg6sbe6agsgris7atkx2iklrhwhsd76xooq.b32.i2p
[2025-02-12T03:25:07Z WARN  is2fp::utils] i2p has not warmed up yet, check wrapper.log
[2025-02-12T03:25:18Z INFO  is2fp::i2p] starting router
[2025-02-12T03:25:19Z INFO  is2fp::i2p] router is warming up, please wait...
[2025-02-12T03:26:07Z WARN  is2fp::utils] i2p has not warmed up yet, check wrapper.log
[2025-02-12T03:26:19Z INFO  is2fp::i2p] router is warming up, please wait...
[2025-02-12T03:27:07Z WARN  is2fp::utils] i2p has not warmed up yet, check wrapper.log
[2025-02-12T03:28:07Z WARN  is2fp::utils] i2p has not warmed up yet, check wrapper.log
[2025-02-12T03:28:19Z INFO  is2fp::i2p] router is running on external port = 24724
[2025-02-12T03:28:19Z INFO  is2fp::i2p] open this port for better connectivity
[2025-02-12T03:28:19Z INFO  is2fp::i2p] this port was randomly assigned, keep it private
[2025-02-12T03:28:21Z INFO  is2fp::i2p] http proxy on port 4242
[2025-02-12T03:28:24Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:29:07Z INFO  is2fp::utils] i2p fluff propagation server online
[2025-02-12T03:29:07Z INFO  is2fp::utils] IS2FP Console v0.1.0-alpha
    
                    add peer /ip4/<IP>/tcp/<PORT>/p2p/<PEER_ID>
    
                    send <MESSAGE>
[2025-02-12T03:29:07Z INFO  is2fp::utils] Local node is listening on /ip4/127.0.0.1/tcp/45755/p2p/12D3KooWLHJWRQvAeWV6f8wXMXGDZZaJfZgnRioo2qrgP89arRmq
[2025-02-12T03:29:07Z INFO  is2fp::utils] Local node is listening on /ip4/192.168.232.16/tcp/45755/p2p/12D3KooWLHJWRQvAeWV6f8wXMXGDZZaJfZgnRioo2qrgP89arRmq
[2025-02-12T03:29:07Z INFO  is2fp::utils] Local node is listening on /ip4/172.16.53.55/tcp/45755/p2p/12D3KooWLHJWRQvAeWV6f8wXMXGDZZaJfZgnRioo2qrgP89arRmq
[2025-02-12T03:43:43Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/38757/p2p/12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX
[2025-02-12T03:43:43Z INFO  is2fp::utils] Connected to peer: PeerId("12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX")
[2025-02-12T03:46:48Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/41565/p2p/12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1
[2025-02-12T03:46:48Z INFO  is2fp::utils] Connected to peer: PeerId("12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1")
[2025-02-12T03:46:51Z INFO  is2fp::utils] anon: tzjg7qddk73vyum5rc4v6ucv6mjksgopbfj32udb6kotzlmawpca.b32.i2p
[2025-02-12T03:46:51Z INFO  is2fp::utils] handling message type: B32Exchange
[2025-02-12T03:46:51Z INFO  is2fp::utils] processing address tzjg7qddk73vyum5rc4v6ucv6mjksgopbfj32udb6kotzlmawpca.b32.i2p for relays
[2025-02-12T03:46:51Z INFO  is2fp::utils] writing new relay:PeerId("12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1") to lmdb
[2025-02-12T03:46:51Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:46:51Z INFO  is2fp::utils] anon: 6irbexwp3bymtvxniaaml3rlmc4klgwmhxzapr5tomgiar46bela.b32.i2p
[2025-02-12T03:46:51Z INFO  is2fp::utils] handling message type: B32Exchange
[2025-02-12T03:46:51Z INFO  is2fp::utils] processing address 6irbexwp3bymtvxniaaml3rlmc4klgwmhxzapr5tomgiar46bela.b32.i2p for relays
[2025-02-12T03:46:51Z INFO  is2fp::utils] writing new relay:PeerId("12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX") to lmdb
[2025-02-12T03:46:51Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:48:40Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/36145/p2p/12D3KooWJeJu6dY1MNk3xdLNfQQ36H53x3vaCEsygZp4T7wGkgkq
[2025-02-12T03:48:43Z INFO  is2fp::utils] anon: nsb7rtjreuuzjoyngcrb7wvctnna5ikr4alqkwyfcfrkjvupiqia.b32.i2p
[2025-02-12T03:48:43Z INFO  is2fp::utils] handling message type: B32Exchange
[2025-02-12T03:48:43Z INFO  is2fp::utils] processing address nsb7rtjreuuzjoyngcrb7wvctnna5ikr4alqkwyfcfrkjvupiqia.b32.i2p for relays
[2025-02-12T03:48:43Z INFO  is2fp::utils] writing new relay:PeerId("12D3KooWJeJu6dY1MNk3xdLNfQQ36H53x3vaCEsygZp4T7wGkgkq") to lmdb
[2025-02-12T03:48:43Z INFO  is2fp::db] excecuting lmdb write
send test message
[2025-02-12T03:50:05Z INFO  is2fp::utils] sending message: test message
[2025-02-12T03:50:05Z INFO  is2fp::utils] start invisible stem selection
[2025-02-12T03:50:05Z INFO  is2fp::utils] connected peers: 3
[2025-02-12T03:50:05Z DEBUG is2fp::utils] random relay: PeerId("12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1")
[2025-02-12T03:50:05Z DEBUG is2fp::utils] pow hash: 5d198d1a14d73df92c73fd5bb3dfc452d820d6727e8302e6d612b09e22856f2f8ec4be26868a088a98c053e63e8316f63b211593a038c103e695e98f6c36b28c
[2025-02-12T03:50:05Z INFO  is2fp::utils] broadcasting message to relay: tzjg7qddk73vyum5rc4v6ucv6mjksgopbfj32udb6kotzlmawpca.b32.i2p
[2025-02-12T03:50:05Z WARN  is2fp::utils] failed to read i2p proxy host: NotPresent
[2025-02-12T03:50:05Z DEBUG is2fp::utils] setting i2p proxy to: http://127.0.0.1:4242
[2025-02-12T03:51:06Z WARN  is2fp::utils] unknown relay status
[2025-02-12T03:57:44Z INFO  is2fp::utils] anon: test message
[2025-02-12T03:57:44Z INFO  is2fp::utils] handling message type: Fluff

```

Bob

```bash
RUST_LOG=none,is2fp=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/is2fp`
[2025-02-12T03:42:43Z INFO  is2fp::utils] dandelion-is2fp is starting up
[2025-02-12T03:42:43Z INFO  is2fp::db] setting lmdb map size to: 1227913600
[2025-02-12T03:42:43Z INFO  is2fp::db] $LMDB_USER=user
[2025-02-12T03:42:43Z INFO  is2fp::db] excecuting lmdb open
[2025-02-12T03:42:43Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:42:43Z WARN  is2fp::utils] failed to read i2p proxy host: NotPresent
[2025-02-12T03:42:43Z INFO  is2fp::i2p] starting j4i2prs...
[2025-02-12T03:42:43Z INFO  is2fp::utils] relay server address - 6irbexwp3bymtvxniaaml3rlmc4klgwmhxzapr5tomgiar46bela.b32.i2p
[2025-02-12T03:42:43Z WARN  is2fp::utils] i2p has not warmed up yet, check wrapper.log
[2025-02-12T03:42:43Z INFO  is2fp::i2p] router is running on external port = 24724
[2025-02-12T03:42:43Z INFO  is2fp::i2p] open this port for better connectivity
[2025-02-12T03:42:43Z INFO  is2fp::i2p] this port was randomly assigned, keep it private
Tunnels ready for server at 127.0.0.1:5556
[2025-02-12T03:42:49Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:43:43Z INFO  is2fp::utils] i2p fluff propagation server online
[2025-02-12T03:43:43Z INFO  is2fp::utils] IS2FP Console v0.1.0-alpha
    
                    add peer /ip4/<IP>/tcp/<PORT>/p2p/<PEER_ID>
    
                    send <MESSAGE>
[2025-02-12T03:43:43Z INFO  is2fp::utils] Local node is listening on /ip4/127.0.0.1/tcp/38757/p2p/12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX
[2025-02-12T03:43:43Z INFO  is2fp::utils] Local node is listening on /ip4/192.168.232.16/tcp/38757/p2p/12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX
[2025-02-12T03:43:43Z INFO  is2fp::utils] Local node is listening on /ip4/172.16.53.55/tcp/38757/p2p/12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX
[2025-02-12T03:43:43Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/45755/p2p/12D3KooWLHJWRQvAeWV6f8wXMXGDZZaJfZgnRioo2qrgP89arRmq
[2025-02-12T03:43:43Z INFO  is2fp::utils] Connected to peer: PeerId("12D3KooWLHJWRQvAeWV6f8wXMXGDZZaJfZgnRioo2qrgP89arRmq")
[2025-02-12T03:46:48Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/41565/p2p/12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1
[2025-02-12T03:46:48Z INFO  is2fp::utils] Connected to peer: PeerId("12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1")
[2025-02-12T03:46:51Z INFO  is2fp::utils] anon: 4cgh4c26jwyr6nuk4lg6sbe6agsgris7atkx2iklrhwhsd76xooq.b32.i2p
[2025-02-12T03:46:51Z INFO  is2fp::utils] handling message type: B32Exchange
[2025-02-12T03:46:51Z INFO  is2fp::utils] processing address 4cgh4c26jwyr6nuk4lg6sbe6agsgris7atkx2iklrhwhsd76xooq.b32.i2p for relays
[2025-02-12T03:46:51Z INFO  is2fp::utils] writing new relay:PeerId("12D3KooWLHJWRQvAeWV6f8wXMXGDZZaJfZgnRioo2qrgP89arRmq") to lmdb
[2025-02-12T03:46:51Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:48:40Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/36145/p2p/12D3KooWJeJu6dY1MNk3xdLNfQQ36H53x3vaCEsygZp4T7wGkgkq
[2025-02-12T03:48:43Z INFO  is2fp::utils] Connected to peer: PeerId("12D3KooWJeJu6dY1MNk3xdLNfQQ36H53x3vaCEsygZp4T7wGkgkq")
[2025-02-12T03:57:44Z INFO  is2fp::utils] anon: test message
[2025-02-12T03:57:44Z INFO  is2fp::utils] handling message type: Fluff

```

Carol - (invisible stem)

```bash
 RUST_LOG=none,is2fp=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/is2fp`
[2025-02-12T03:45:48Z INFO  is2fp::utils] dandelion-is2fp is starting up
[2025-02-12T03:45:48Z INFO  is2fp::db] setting lmdb map size to: 1224925184
[2025-02-12T03:45:48Z INFO  is2fp::db] $LMDB_USER=user
[2025-02-12T03:45:48Z INFO  is2fp::db] excecuting lmdb open
[2025-02-12T03:45:48Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:45:48Z WARN  is2fp::utils] failed to read i2p proxy host: NotPresent
[2025-02-12T03:45:48Z INFO  is2fp::i2p] starting j4i2prs...
[2025-02-12T03:45:48Z INFO  is2fp::utils] relay server address - tzjg7qddk73vyum5rc4v6ucv6mjksgopbfj32udb6kotzlmawpca.b32.i2p
[2025-02-12T03:45:48Z WARN  is2fp::utils] i2p has not warmed up yet, check wrapper.log
[2025-02-12T03:45:48Z INFO  is2fp::i2p] router is running on external port = 24724
[2025-02-12T03:45:48Z INFO  is2fp::i2p] open this port for better connectivity
[2025-02-12T03:45:48Z INFO  is2fp::i2p] this port was randomly assigned, keep it private
Tunnels ready for server at 127.0.0.1:5557
[2025-02-12T03:45:53Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:46:48Z INFO  is2fp::utils] i2p fluff propagation server online
[2025-02-12T03:46:48Z INFO  is2fp::utils] IS2FP Console v0.1.0-alpha
    
                    add peer /ip4/<IP>/tcp/<PORT>/p2p/<PEER_ID>
    
                    send <MESSAGE>
[2025-02-12T03:46:48Z INFO  is2fp::utils] Local node is listening on /ip4/127.0.0.1/tcp/41565/p2p/12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1
[2025-02-12T03:46:48Z INFO  is2fp::utils] Local node is listening on /ip4/192.168.232.16/tcp/41565/p2p/12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1
[2025-02-12T03:46:48Z INFO  is2fp::utils] Local node is listening on /ip4/172.16.53.55/tcp/41565/p2p/12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1
[2025-02-12T03:46:48Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/45755/p2p/12D3KooWLHJWRQvAeWV6f8wXMXGDZZaJfZgnRioo2qrgP89arRmq
[2025-02-12T03:46:48Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/38757/p2p/12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX
[2025-02-12T03:46:48Z INFO  is2fp::utils] Connected to peer: PeerId("12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX")
[2025-02-12T03:46:51Z INFO  is2fp::utils] anon: 4cgh4c26jwyr6nuk4lg6sbe6agsgris7atkx2iklrhwhsd76xooq.b32.i2p
[2025-02-12T03:46:51Z INFO  is2fp::utils] handling message type: B32Exchange
[2025-02-12T03:46:51Z INFO  is2fp::utils] processing address 4cgh4c26jwyr6nuk4lg6sbe6agsgris7atkx2iklrhwhsd76xooq.b32.i2p for relays
[2025-02-12T03:46:51Z INFO  is2fp::utils] writing new relay:PeerId("12D3KooWLHJWRQvAeWV6f8wXMXGDZZaJfZgnRioo2qrgP89arRmq") to lmdb
[2025-02-12T03:46:51Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:48:40Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/36145/p2p/12D3KooWJeJu6dY1MNk3xdLNfQQ36H53x3vaCEsygZp4T7wGkgkq
[2025-02-12T03:48:40Z INFO  is2fp::utils] Connected to peer: PeerId("12D3KooWJeJu6dY1MNk3xdLNfQQ36H53x3vaCEsygZp4T7wGkgkq")
[2025-02-12T03:50:06Z INFO  is2fp::utils] injecting fluff msg: 950b2a7effa78f51a63515ec45e03ecebe50ef2f1c41e69629b50778f11bc080002e4db8112b59d09389d10f3558f85bfdeb4f1cc55a34217af0f8547700ebf3
[2025-02-12T03:50:06Z INFO  is2fp::utils] begin pow for : 5d198d1a14d73df92c73fd5bb3dfc452d820d6727e8302e6d612b09e22856f2f8ec4be26868a088a98c053e63e8316f63b211593a038c103e695e98f6c36b28c
[2025-02-12T03:57:44Z INFO  is2fp::utils] found solution to: 5d198d1a14d73df92c73fd5bb3dfc452d820d6727e8302e6d612b09e22856f2f8ec4be26868a088a98c053e63e8316f63b211593a038c103e695e98f6c36b28c
[2025-02-12T03:57:44Z INFO  is2fp::db] excecuting lmdb delete
[2025-02-12T03:57:44Z INFO  is2fp::db] excecuting lmdb write

```

David

```bash
RUST_LOG=none,is2fp=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/is2fp`
[2025-02-12T03:47:40Z INFO  is2fp::utils] dandelion-is2fp is starting up
[2025-02-12T03:47:40Z INFO  is2fp::db] setting lmdb map size to: 1217948032
[2025-02-12T03:47:40Z INFO  is2fp::db] $LMDB_USER=user
[2025-02-12T03:47:40Z INFO  is2fp::db] excecuting lmdb open
[2025-02-12T03:47:40Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:47:40Z WARN  is2fp::utils] failed to read i2p proxy host: NotPresent
[2025-02-12T03:47:40Z INFO  is2fp::i2p] starting j4i2prs...
[2025-02-12T03:47:40Z INFO  is2fp::utils] relay server address - nsb7rtjreuuzjoyngcrb7wvctnna5ikr4alqkwyfcfrkjvupiqia.b32.i2p
[2025-02-12T03:47:40Z WARN  is2fp::utils] i2p has not warmed up yet, check wrapper.log
[2025-02-12T03:47:40Z INFO  is2fp::i2p] router is running on external port = 24724
[2025-02-12T03:47:40Z INFO  is2fp::i2p] open this port for better connectivity
[2025-02-12T03:47:40Z INFO  is2fp::i2p] this port was randomly assigned, keep it private
Tunnels ready for server at 127.0.0.1:5558
[2025-02-12T03:48:09Z INFO  is2fp::db] excecuting lmdb write
[2025-02-12T03:48:40Z INFO  is2fp::utils] i2p fluff propagation server online
[2025-02-12T03:48:40Z INFO  is2fp::utils] IS2FP Console v0.1.0-alpha
    
                    add peer /ip4/<IP>/tcp/<PORT>/p2p/<PEER_ID>
    
                    send <MESSAGE>
[2025-02-12T03:48:40Z INFO  is2fp::utils] Local node is listening on /ip4/127.0.0.1/tcp/36145/p2p/12D3KooWJeJu6dY1MNk3xdLNfQQ36H53x3vaCEsygZp4T7wGkgkq
[2025-02-12T03:48:40Z INFO  is2fp::utils] Local node is listening on /ip4/192.168.232.16/tcp/36145/p2p/12D3KooWJeJu6dY1MNk3xdLNfQQ36H53x3vaCEsygZp4T7wGkgkq
[2025-02-12T03:48:40Z INFO  is2fp::utils] Local node is listening on /ip4/172.16.53.55/tcp/36145/p2p/12D3KooWJeJu6dY1MNk3xdLNfQQ36H53x3vaCEsygZp4T7wGkgkq
[2025-02-12T03:48:40Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/41565/p2p/12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1
[2025-02-12T03:48:40Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/38757/p2p/12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX
[2025-02-12T03:48:40Z INFO  is2fp::utils] mDNS discovered a new stem: /ip4/172.16.53.55/tcp/45755/p2p/12D3KooWLHJWRQvAeWV6f8wXMXGDZZaJfZgnRioo2qrgP89arRmq
[2025-02-12T03:48:40Z INFO  is2fp::utils] Connected to peer: PeerId("12D3KooWBHKBNQyUBnp67PeW9BfAuPHL4CP8imrdWiLK63Ndaei1")
[2025-02-12T03:48:43Z INFO  is2fp::utils] Connected to peer: PeerId("12D3KooWKkRhkJtW8xa7D4cEPnCUqpebUpjo5qZek1PzoFsNqoaX")
[2025-02-12T03:57:44Z INFO  is2fp::utils] anon: test message
[2025-02-12T03:57:44Z INFO  is2fp::utils] handling message type: Fluff

```
