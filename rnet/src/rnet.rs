use serde::Serialize;
use super::RnetSerde;

#[derive(Serialize)]
pub struct Rnet<P: RnetSerde> {
    pub kind: u8,
    pub payload: P
}

impl<P: RnetSerde> Rnet<P> {
    pub fn new(payload: P, pkind: u8) -> Self {
        Self {
            kind: pkind,
            payload
        }
    }
}