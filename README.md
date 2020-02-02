# Rnet 
Rnet is a Rust library allowing to easily exchange generic rust `structs` as udp packets.

A very basic yet performant protobuf-like using only rust internals.
Uses `serde` crate to de/serialize structures.
`SocketConnection` is a `std::net::UdpSocket` wrapper.

## Explanations and usage

#### Library side
a given `struct` is seralized, sent to a server that retrieves it and deserializes it back into the same `struct` using a payload_id matching a defined enum struct.

#### End user side
create a `SocketConnection` to a server, and call the send method directly with a declared struct (from the `components` pool) as a param. That is it. 

#### Dev side
add a new `struct` to the *components/payloads* folder, and add to the `payloads` module the struct name to the enum and to the `match` pattern in `dispatcher`
Any struct must derive the following: `#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]`

Note that if no method is required to a newly defined structure, the `action` method must be re-implemented for the structure to do anything more than printing the received packet

## Workspace architecture

#### This workspace is a test environment.
`rnet` is the only library crate, must be users binary dependency.
`rnet_serde` is a `rnet` lib dependency.
`rnet_serde_derive` is a `rnet_serde` lib dependency.
`components` is an example and setup crate that is no dependency of `rnet`. It should be cloned and updated by users directly. Must be a bin dependency or be part of a client/server workspace

#### Production environment would look like:
Server / client would be 2 separated workspaces. They'd have some libraries (like rnet) and a duplicate of several services: Payloads, GameData, GameLogic. They'd be related to payloads by the action method.
`Cargo.toml` from main binary crate
```
// this is your main crate (can either be a client or a server)
[dependencies]
bincode = "X.X.X" // library crate
components = { path = "../components" } // clone it and update it to handle more payloads
```
Note: `rnet_payload` own git clone link soon(ish), for now : `svn export https://github.com/gpoblon/rnet/components`

## TODO
- Rnet: 
Add several optional abilities like:
Fragmentation
Bufferization
Ordering
Multithreading capability
TCP version
(timestamp takes some bytes and cpu cycles but may be a simple solution for ordering, fragmentation, bufferization)