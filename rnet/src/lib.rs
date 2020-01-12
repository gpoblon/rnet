// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

pub use rnet_serde::*;

mod socket;
pub use socket::SocketConnection as SocketConnection;

mod payloads;
pub use payloads::action_dispatcher as action_dispatcher;
pub use payloads::*;

mod err;
pub use err::SError as SError;
pub use err::SErrorKind as SErrorKind;

