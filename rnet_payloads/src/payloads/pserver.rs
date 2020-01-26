// payload example

use super::RnetSerde;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]
pub struct PServer {
    pub state: u8,
}