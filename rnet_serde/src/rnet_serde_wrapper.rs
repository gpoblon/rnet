use serde::Serialize;
use super::RnetSerde;

#[derive(Serialize)]
pub struct WRnetSerde<'src, P: RnetSerde> {
    pub kind: u8,
    pub payload: &'src P
}

impl<'src, P: RnetSerde> WRnetSerde<'src, P> {
    pub fn new(payload: &'src P, pkind: u8) -> Self {
        Self {
            kind: pkind,
            payload: payload
        }
    }
}