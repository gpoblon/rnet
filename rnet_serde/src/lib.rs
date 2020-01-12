// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

pub use rnet_serde_derive::RnetSerde;

// pub struct Rnet<P: RnetSerde> {
//     pub kind: u8,
//     pub payload: P
// }

// impl<P: RnetSerde> Rnet<P> {
//     fn new(payload: P) -> Self {
//         Self {
//             kind: 0,
//             payload
//         }
//     }
// }

pub trait RnetSerde {
    fn new() -> Self;
    fn serialize(&self) -> Vec<u8>;
    fn from_bytes<'de>(bytes: &'de [u8]) -> Self;
    fn as_bytes(&self) -> Vec<u8>;
    fn action(datagram: &[u8]);
}