// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>
use std::io;
use async_std::net::{UdpSocket, ToSocketAddrs};
use async_std::task::block_on;
use std::cell::RefCell;

use crate::RnetSerde;
use crate::err::{SocketError, SocketErrorKind};

// 512 is the minimum datagram size supposed to be handled by every support:
// minimum reassembly buffer size is 576 for ipv4.
// Sub UDP head (8), ipv4 header (20), up to 60, plus eventually some headers depending on the MTU
// That leaves us 512 which is considered the largest safe datagram size
// TODO : lazy so it is not recreated every time
// TODO give possibility to have a user defined size (<= 512). 
const DATAGRAM_SIZE: usize = 512;
const MIN_PACKET_SIZE: usize = 2; // tmp, wil grow : at least check sum + pkind...

pub struct NetworkSetup<A: ToSocketAddrs> {
    local_addr: A,
    remote_addr: A,
    pub packets_version: [u8;3]
}

pub struct SocketConnection {
    datagram: RefCell<[u8; DATAGRAM_SIZE]>,
    socket: UdpSocket,
    packets_version: [u8;3]
}

impl SocketConnection {
    /// Contains data that should be stored elsewhere, and crypted
    /// Storing these values serialized on a remote db synchronised with the server seems like a good way to do it
    /// will later be an autonomous call to the server, `pub` attribute and parameters wil disappear
    pub fn prepare<A: ToSocketAddrs>(local_addr: A, remote_addr: A, packets_version: [u8;3]) -> NetworkSetup<A> {
        NetworkSetup {
            local_addr,
            remote_addr,
            packets_version
        }
    }

    pub fn new<A: ToSocketAddrs>(config: &NetworkSetup<A>) -> io::Result<Self> {
        let socket = block_on(UdpSocket::bind(&config.local_addr))?;
        block_on(socket.connect(&config.remote_addr))?;
        Ok(Self {
            datagram: RefCell::new([0; DATAGRAM_SIZE]),
            socket,
            packets_version: config.packets_version
        })
    }

    pub async fn send<'de, P: RnetSerde>(&'de self, payload: &P) -> io::Result<()> {
        let size = self.socket.send(&payload.prepare(self.packets_version)[..]).await?;
        if size > DATAGRAM_SIZE {                                                                                                                                                                                                 
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, SocketError::msg(SocketErrorKind::DatagramTooLarge)))
        }
        // println!("Packet Sent");
        Ok(())
    }

    pub async fn recv(&self) -> io::Result<usize> {
        let size = self.socket.recv(&mut *self.datagram.borrow_mut()).await?;
        // println!("Packet Received");
        if size > DATAGRAM_SIZE {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, SocketError::msg(SocketErrorKind::DatagramTooLarge)))
        } else if size < MIN_PACKET_SIZE {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, SocketError::msg(SocketErrorKind::DatagramTooSmall)))
        }
        Ok(size)
    }

    pub fn get_datagram(&self) -> [u8; DATAGRAM_SIZE] {
        *self.datagram.borrow()
    }
}
