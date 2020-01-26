// update this file to add new handled structs from the payloads folder

// The following derive macros are mandatory for any payload struct
// #[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]

use super::RnetSerde;

mod pclient;
pub use pclient::PClient;

mod pserver;
pub use pserver::PServer;

#[derive(Debug)]
pub enum PayloadKind {
    PClient,
    PServer,
}

pub fn dispatcher(datagram: &[u8]) {
    let pkind = datagram[0];
    match pkind {
        0 => PClient::action(datagram),
        1 => PServer::action(datagram),
        _ => panic!("unhandled payload")
    };
}