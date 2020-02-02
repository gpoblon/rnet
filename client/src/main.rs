// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2020 Gaëtan Poblon <gaetan.poblon@gmail.com>

use rnet::*;
use components::*;
use async_std::task::block_on;

use std::thread;
use std::time;

const REMOTE_ADDR: &str = "127.0.0.1:8079";
const LOCAL_ADDR: &str = "127.0.0.1:8081";
const PACKETS_VERSION: [u8; 3] = [0,1,0]; // should be stored online (db or something)

async fn run_server(sc: &SocketConnection, packets_version: [u8;3]) {
    let mut paction = PlayerAction { ..Default::default() };
    let pnew = PlayerNew { name: String::from("Faith"), ..Default::default() };
    &sc.send(&paction).await.unwrap();
    &sc.send(&pnew).await.unwrap();
    // match sc.recv().await {
    //     Ok(_) => match dispatcher(&sc.get_datagram(), packets_version) {
    //         Ok(res) => println!("Packet recvd result: {:#?}", res),
    //         Err(e) => {
    //             println!("Error recvd: {}", e.explanation);
    //             if !e.is_recoverable {
    //                 sc.send(&e).await.unwrap();
    //             }
    //         }
    //     },
    //     Err(e) => panic!("Packet recv error: {:#?}", e)
    // }
    // thread::sleep(time::Duration::from_millis(2000)); // frame_time. OW netcode is based on quantized 16ms, 7ms if tournament 
}


fn main() {
    let network_setup = SocketConnection::prepare(LOCAL_ADDR, REMOTE_ADDR, PACKETS_VERSION);
    let packets_version = network_setup.packets_version;
    let sc = SocketConnection::new(&network_setup).expect(SocketError::msg(SocketErrorKind::SocketConnection));
    loop {
        block_on(run_server(&sc, packets_version));
    }
}
// BE CAREFUL, if `datagram` is too short -> crash of bincode