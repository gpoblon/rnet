// update this file to add new handled structs from the payloads folder

// The following derive macros are mandatory for any payload struct
// #[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]

use super::rnet_serde::*;

mod player;
pub use player::*;
mod error;
pub use error::RnetError;

// not sent through network but is used as the standard Result return type for any payload
pub type RnetResult = std::result::Result<(), RnetError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum PayloadKind {
    RnetError,
    PlayerAction,
    PlayerNew,
}

pub fn dispatcher(datagram: &[u8], local_version: [u8;3]) -> RnetResult {
    let header = WRnetHeader::from(datagram);
    let remote_version = header.version.as_slice();
    if remote_version == local_version {
        return match header.payload_kind {
            0 => RnetError::action(datagram),
            1 => PlayerAction::action(datagram),
            2 => PlayerNew::action(datagram),
            _ => {
                // actually this might mean that packet is corrupted and should end connection
                Err(RnetError::new(None, "dispatcher", "Payload kind not recognized", true))
            }
        }
    } else {
        Err(RnetError::new(None, "dispatcher", "Packet version differ, must end connection", false))
    }
}