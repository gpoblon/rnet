use serde::Serialize;
use super::RnetSerde;

#[derive(Serialize)]
pub struct WRnetSerde<P: RnetSerde> {
    pub kind: u8,
    pub payload: P
}

impl<P: RnetSerde> WRnetSerde<P> {
    pub fn new(payload: P, pkind: u8) -> Self {
        Self {
            kind: pkind,
            payload
        }
    }
}