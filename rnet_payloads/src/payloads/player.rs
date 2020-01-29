// payload example
use super::*;

use crate::skills::Skill;
use crate::player::{Faction, Gender, Art};

#[derive(Default, Debug, Serialize, Deserialize, RnetSerde)]
pub struct PlayerAction {
    pub position: (u8, u8),
    pub lookat: u8,
    pub state: u64,
    pub skill: Skill,
}

impl PlayerAction {
    pub fn action(datagram: &[u8])
    where Self: std::fmt::Debug + Sized
    {
        let ser: Self = Self::from_bytes(datagram);
        println!("From PlayerAction action: {:#?}", ser);
    }
}

#[derive(Default, Debug, Serialize, Deserialize, RnetSerde)]
pub struct PlayerNew {
    pub name: String,
    pub faction: Faction,
    pub gender: Gender,
    // role: Role,
    pub art: Art
}