Rnet is a Rust library allowing to easily exchange generic rust `structs` as udp packets.

A very basic yet performant protobuf-like using only rust internals.
Uses `serde` crate to de/serialize structures.
`SocketConnection` is a `std::net::UdpSocket` wrapper.

Library side, a given `struct` is seralized, sent to a server that retrieves it and deserializes it back into the same `struct` using a payload_id matching a defined enum struct.

End user side, create a `SocketConnection` to a server, and call the send method directly with a declared struct (from the `payloads` pool) as a param. That is it. 

Dev side : add a new `struct` to the *rnet/src/payloads* folder, and add to the `payloads` module the struct name to the enum and to the `match` pattern in `dispatcher`
// TODO import external crate structs if possible
Any struct must derive the following: `#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]`

Note that if no method is required to a newly defined structure, the `action` method must be re-implemented for the structure to do anything more than printing the received packet