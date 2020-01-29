// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

#![allow(dead_code)]
use std::fmt;
use colored::Colorize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SocketErrorKind {
    DatagramTooLarge,
    DatagramTooSmall,
    PacketNotReceived,
    PacketNotSent,
    SocketAddrFormat,
    SocketBinding,
    SocketConnection,
}

pub struct SocketError {
    pub kind: SocketErrorKind,
}
impl SocketError {
    pub fn msg(error_kind: SocketErrorKind) -> &'static str {
        match error_kind {
            SocketErrorKind::DatagramTooLarge => "Datagram size is bigger than its container (DATAGRAM_SIZE)",
            SocketErrorKind::DatagramTooSmall => "Datagram size is smaller than the required payload header size",
            SocketErrorKind::PacketNotReceived => "Could not retrieve packet from peer (probably not connected)",
            SocketErrorKind::PacketNotSent => "Failed to send packet to connected peer",
            SocketErrorKind::SocketAddrFormat => "Unrecognized host address format",
            SocketErrorKind::SocketBinding => "Failed to bind to address",
            SocketErrorKind::SocketConnection => "Failed to connect to address",
        }
    }
}
impl fmt::Display for SocketError {
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