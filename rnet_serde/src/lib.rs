// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

pub use rnet_serde_derive::RnetSerde;

mod rnet_serde_wrapper;
pub use rnet_serde_wrapper::{WRnetSerde, WRnetHeader, WRnetVersion};

pub use serde::{Serialize, Deserialize};

pub trait RnetSerde {
    // fn new() -> Self; // maybe new should not be mandatory as it forces Default
    fn as_ref(&self) -> &Self;
    fn payload_from_bytes<'de>(bytes: &'de [u8]) -> Self;
    fn prepare(&self, version: [u8;3]) -> Vec<u8>;
    fn debug(&self);
    fn action(datagram: &[u8]);
}