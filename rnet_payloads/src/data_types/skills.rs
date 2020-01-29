use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Skill {
    None,
    Invincible,
    SwordSlash,
    ArrowShot,
    Firebolt,
}
impl Default for Skill { fn default() -> Self { Skill::None } }