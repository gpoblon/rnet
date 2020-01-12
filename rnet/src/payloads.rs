// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

// The following derive macros are mandatory for any payload struct
// #[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]

use super::RnetSerde;

mod pclient;
pub use pclient::PClient;

mod pserver;
pub use pserver::PServer;

// not in use for now but could be used to match payload index
pub enum PayloadKind {
    PClient,
    PServer,
}

macro_rules! payload_deserializer {
    ($payload_id:expr, $datagram:expr) => {
        match $payload_id {
            0 => PClient::action($datagram),
            1 => PServer::action($datagram),
            _ => panic!("unhandled payload")
        };
    }
}

pub fn action_dispatcher(payload_id: u8, datagram: &[u8]) {
    println!("payload_id = {}", payload_id);
    payload_deserializer!(payload_id, datagram);
}