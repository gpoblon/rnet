// use postgres_types::{ToSql, FromSql};
use crate::payloads::PlayerNew;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Class and build relative entities
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Skill {
    None,
    Invincible,
    SwordSlash,
    ArrowShot,
    Firebolt,
}
impl Default for Skill { fn default() -> Self { Skill::None } }

#[derive(Default, Debug)]
struct CEquipement {
    weapon: Option<String>, // tmp before giving it a type
    armor: Option<String>, // tmp before giving it a type
}

// Character identity relative entities
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

#[derive(Default, Debug)]
struct CStats {
    health: u16,
    mana: u16,
    power: u16,
    defense: u16,
}

#[derive(Default, Debug)]
pub struct GContainer<'src> {
    objects: HashMap<&'src GObject, u8>,
}

#[derive(Default, Debug, Hash, Eq, PartialEq)]
pub struct GObject {
    name: String
}

#[derive(Debug)]
pub struct GCharacter<'src> {
    name: String,
    gender: Gender,
    faction: Faction,
    role: Role,
    guild: Option<String>,
    stats: CStats,
    equipement: CEquipement,
    inventory: GContainer<'src>,
}
impl<'src> GCharacter<'src> {
    pub fn new(name: String, gender: Gender, faction: Faction, role: Role) -> Self {
        Self {
            name,
            gender,
            faction,
            role,
            guild: None,
            stats: CStats::default(),
            equipement: CEquipement::default(),
            inventory: GContainer::default(),
        }
    }
}