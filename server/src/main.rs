// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

use rnet::*;

use std::thread;
use std::time;

const SERVER_ADDR: &str = "127.0.0.1:8081";
const PEER_ADDR: &str = "127.0.0.1:8079";

fn main() {
    let sc = SocketConnection::new(SERVER_ADDR, PEER_ADDR, false).expect(SError::msg(SErrorKind::SocketConnection));
    loop {
        match sc.recv() {
            Ok(_) => action_dispatcher(sc.get_payload_id(), &sc.get_datagram()),
            Err(e) => panic!("Packet recv error: {:#?}", e)
        }
        thread::sleep(time::Duration::from_millis(500)); // frame_time. OW netcode is based on quantized 16ms, 7ms if tournament 
    };
}
// BE CAREFUL, if `datagram` is too short -> crash of bincode