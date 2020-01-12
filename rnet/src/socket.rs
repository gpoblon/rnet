// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

use std::io;
use std::net::{UdpSocket, ToSocketAddrs};
use std::cell::RefCell;

use crate::RnetSerde;
use crate::err::{SError, SErrorKind};

// 512 is the minimum datagram size supposed to be handled by every support:
// minimum reassembly buffer size is 576 for ipv4.
// Sub UDP head (8), ipv4 header (20), up to 60, plus eventually some headers depending on the MTU
// That leaves us 512 which is considered the largest safe datagram size
// TODO : lazy so it is not recreated every time
// TODO give possibility to have a user defined size (<= 512). 
const DATAGRAM_SIZE: usize = 512;

pub struct SocketConnection {
    datagram: RefCell<[u8; DATAGRAM_SIZE]>,
    socket: UdpSocket,
}

impl SocketConnection {
    pub fn new<A: ToSocketAddrs>(from_addr: A, to_addr: A, is_blocking: bool) -> io::Result<Self> {
        let socket = UdpSocket::bind(from_addr)?;
        socket.connect(&to_addr)?;
        socket.set_nonblocking(is_blocking)?;
        Ok(Self {
            datagram: RefCell::new([0; DATAGRAM_SIZE]),
            socket,
        })
    }

    pub fn send<'de, P: RnetSerde>(&'de self, payload: P) -> io::Result<()> {
        let size = self.socket.send(&payload.serialize()[..])?;
        if size > DATAGRAM_SIZE {                                                                                                                                                                                                 
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, SError::msg(SErrorKind::DatagramTooLarge)))
        }
        Ok(())
    }

    pub fn recv(&self) -> io::Result<usize> {
        let size = self.socket.recv(&mut *self.datagram.borrow_mut())?;
        // println!("payload: {:?}", self.datagram.borrow());
        if size > DATAGRAM_SIZE {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, SError::msg(SErrorKind::DatagramTooLarge)))
        } else if size < 2 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, SError::msg(SErrorKind::DatagramTooSmall)))
        }
        Ok(size)
    }

    pub fn get_datagram(&self) -> [u8; DATAGRAM_SIZE] {
        *self.datagram.borrow()
    }

    pub fn get_payload_id(&self) -> u8 {
        self.datagram.borrow()[0]
    }
}
