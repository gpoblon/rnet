/// To add a field to every payload header, just add it here
/// (and if needed from the proc_macro rnet_serde_derive)
/// Then it will be passed to the dispatcher header variable

use serde::Serialize;
use super::RnetSerde;

#[derive(Serialize, PartialEq, Debug)]
pub struct WRnetVersion {
    major: u8,
    minor: u8,
    patch: u8
}
impl WRnetVersion {
    fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self { major, minor, patch }
    }

    pub fn as_slice(&self) -> [u8; 3] {
        [ self.major, self.minor, self.patch ]
    }
}
impl From<&[u8]> for WRnetVersion {
    fn from(datagram: &[u8]) -> Self {
        Self {
            major: datagram[0],
            minor: datagram[1],
            patch: datagram[2]
        }
    }
}

#[derive(Serialize, Debug)]
pub struct WRnetHeader {
    pub version: WRnetVersion,
    pub payload_kind: u8,
}
impl WRnetHeader {
    pub fn from(datagram: &[u8]) -> Self {
        Self {
            version: WRnetVersion::from(datagram),
            payload_kind: datagram[3]
        }
    }
}


#[derive(Serialize)]
pub struct WRnetSerde<'src, P: RnetSerde> {
    pub header: WRnetHeader,
    pub payload: &'src P
}

impl<'src, P: RnetSerde> WRnetSerde<'src, P> {
    pub fn new(payload: &'src P, payload_kind: u8, version: [u8; 3]) -> Self {
        Self {
            header: WRnetHeader{
                version: WRnetVersion::new(version[0], version[1], version[2]),
                payload_kind,
            },
            payload: payload
        }
    }
}