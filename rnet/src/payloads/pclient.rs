// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

use super::RnetSerde;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]
pub struct PClient {
    pub position: (u8, u8),
    pub lookat: u8,
    pub state: u64,
    pub skill: SkillId,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum SkillId {
    None,
    Invincible,
    SwordSlash,
    ArrowShot,
    Firebolt,
}

impl Default for SkillId {
    fn default() -> Self {
        SkillId::None
    }
}
