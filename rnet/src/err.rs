// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

#![allow(dead_code)]
use std::fmt;
use colored::Colorize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SErrorKind {
    DatagramTooLarge,
    DatagramTooSmall,
    PacketNotReceived,
    PacketNotSent,
    SocketAddrFormat,
    SocketBinding,
    SocketConnection,
}

pub struct SError {
    pub kind: SErrorKind,
}
impl SError {
    pub fn msg(error_kind: SErrorKind) -> &'static str {
        match error_kind {
            SErrorKind::DatagramTooLarge => "Datagram size is bigger than its container (DATAGRAM_SIZE)",
            SErrorKind::DatagramTooSmall => "Datagram size is smaller than the required payload header size",
            SErrorKind::PacketNotReceived => "Could not retrieve packet from peer (probably not connected)",
            SErrorKind::PacketNotSent => "Failed to send packet to connected peer",
            SErrorKind::SocketAddrFormat => "Unrecognized host address format",
            SErrorKind::SocketBinding => "Failed to bind to address",
            SErrorKind::SocketConnection => "Failed to connect to address",
        }
    }
}
impl fmt::Display for SError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = Self::msg(self.kind);
        f.write_str(&format!(
            "{}\n{} {}",
            "Socket error".red(),
            "-->".bright_blue(),
            message.bold(),
        ))
    }
}