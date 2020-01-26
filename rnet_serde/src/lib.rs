// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

pub use rnet_serde_derive::RnetSerde;

pub trait RnetSerde {
    fn new() -> Self;
    fn from_bytes<'de>(bytes: &'de [u8]) -> Self;
    fn as_bytes(&self) -> Vec<u8>;
    fn action(datagram: &[u8]);
}