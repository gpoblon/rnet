use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Faction {
    Libre,
    Chronien,
    Amar
}
impl Default for Faction { fn default() -> Self { Faction::Libre } }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female
}
impl Default for Gender { fn default() -> Self { Gender::Male } }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Damage,
    Healer,
    Tank
}
impl Default for Role { fn default() -> Self { Role::Tank } }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Art {
    Templar,
    Warrior,
    Archer,
    Assassin,
    Mage,
    Necromancer,
    WhiteMage,
}
impl Default for Art { fn default() -> Self { Art::Templar } }
