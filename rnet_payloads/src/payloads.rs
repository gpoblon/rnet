// update this file to add new handled structs from the payloads folder

// The following derive macros are mandatory for any payload struct
// #[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]

use super::{RnetSerde, WRnetSerde};
use serde::{Serialize, Deserialize};

mod player;
pub use player::*;
mod error;
pub use error::RnetError;

// not sent through network but is used as the standard Result return type for any payload
pub type RnetResult<P> = std::result::Result<P, RnetError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum PayloadKind {
    RnetError,
    PlayerAction,
    PlayerNew,
}

pub fn dispatcher(datagram: &[u8]) {
    let pkind = datagram[0];
    match pkind {
        0 => RnetError::action(datagram),
        1 => PlayerAction::action(datagram),
        2 => PlayerNew::action(datagram),
        _ => panic!("unhandled payload")
    };
}