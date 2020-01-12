// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

use super::RnetSerde;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, RnetSerde)]
pub struct PServer {
    pub state: u8,
}