// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

use rnet::*;
use components::*;

use std::thread;
use std::time;

const REMOTE_ADDR: &str = "127.0.0.1:8079";
const LOCAL_ADDR: &str = "127.0.0.1:8081";
const PACKETS_VERSION: [u8; 3] = [0,1,0]; // should be stored online (db or something)

fn main() {
    let network_setup = SocketConnection::prepare(LOCAL_ADDR, REMOTE_ADDR, PACKETS_VERSION);
    let packets_version = network_setup.packets_version;
    let sc = SocketConnection::new(&network_setup, false).expect(SocketError::msg(SocketErrorKind::SocketConnection));
    let mut paction = PlayerAction { ..Default::default() };
    let pnew = PlayerNew { name: String::from("Faith"), ..Default::default() };
    loop {
        // TODO: recv 
        println!("sending...");
        paction.lookat += 1;
        sc.send(&paction).unwrap();
        sc.send(&pnew).unwrap();
        thread::sleep(time::Duration::from_millis(2000)); // frame_time. OW netcode is based on quantized 16ms, 7ms if tournament 
    };

}
// BE CAREFUL, if `datagram` is too short -> crash of bincode