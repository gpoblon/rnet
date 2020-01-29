// update this file to add new handled structs from the payloads folder

// The following derive macros are mandatory for any payload struct
// #[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]

use super::{RnetSerde, WRnetSerde};
use serde::{Serialize, Deserialize};

mod player;
pub use player::*;

#[derive(Debug)]
pub enum PayloadKind {
    PlayerAction,
    PlayerNew,
}

pub fn dispatcher(datagram: &[u8]) {
    let pkind = datagram[0];
    match pkind {
        0 => PlayerAction::action(datagram),
        1 => PlayerNew::action(datagram),
        _ => panic!("unhandled payload")
    };
}