// payload example
use super::*;
use crate::entities::character::*;
use crate::{ RnetResult, RnetError };

#[derive(Default, Debug, Serialize, Deserialize, RnetSerde)]
pub struct PlayerAction {
    pub position: (u8, u8),
    pub lookat: u8,
    pub skill: Skill,
}
impl PlayerAction {
    pub fn action(datagram: &[u8]) -> RnetResult
    where Self: std::fmt::Debug + Sized
    {
        let ser: Self = Self::payload_from_bytes(datagram);
        ser.debug();
        Ok(None)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, RnetSerde)]
pub struct PlayerNew {
    pub name: String,
    pub gender: Gender,
    pub faction: Faction,
    pub role: Role,
}
impl PlayerNew {
    // action triggered when receiving a PlayerAction
    pub fn action(datagram: &[u8]) -> RnetResult
    where Self: std::fmt::Debug + Sized
    {
        let player: Self = Self::payload_from_bytes(datagram);
        let character = GCharacter::new(
            player.name,
            player.gender,
            player.faction,
            player.role,
        );
        println!("new player created: {:?}", character);
        Ok(None)
    }
}