// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

pub use rnet_serde::RnetSerde;

mod socket;
pub use socket::SocketConnection;

mod err;
pub use err::{ SError, SErrorKind };

