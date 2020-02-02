// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

use rnet::*;
use components::*;

use std::thread;
use std::time::{Duration, Instant};

const REMOTE_ADDR: &str = "127.0.0.1:8081";
const LOCAL_ADDR: &str = "127.0.0.1:8079";
const PACKETS_VERSION: PacketVersion = [0,1,0]; // should be stored online (db or something)

fn receiver(sc: &SocketConnection, packets_version: PacketVersion) -> RnetResult {
    match sc.recv() {
        Ok(_) => match dispatcher(&sc.get_datagram(), packets_version) {
            Ok(res) => println!("Packet Action result: {:#?}", res),
            Err(e) => {
                println!("Packet Action error: {:#?}", e);
                if !e.is_recoverable {
                    sc.send(&e);
                    return Err(e);
                }
            }
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
        Err(e) => panic!("Packet recv error: {:#?}", e)
    }
    Ok(()) // will later return a serialized component eventually, to use in game logic
}

fn main() {
    let network_setup = SocketConnection::prepare(LOCAL_ADDR, REMOTE_ADDR, PACKETS_VERSION);
    let packets_version = network_setup.packets_version;
    let sc = &SocketConnection::new(&network_setup, true).expect(SocketError::msg(SocketErrorKind::SocketConnection));
    let mut paction = PlayerAction { ..Default::default() };
    loop {
        if let Err(e) = receiver(&sc, packets_version) { break; }
        sc.send(&paction).unwrap();
        thread::sleep(Duration::from_millis(1000)); // frame_time. OW netcode is based on quantized 16ms, 7ms if tournament 
    };
}