use rnet::*;
use rnet_payloads::*;

use std::thread;
use std::time;

const SERVER_ADDR: &str = "127.0.0.1:8081";
const PEER_ADDR: &str = "127.0.0.1:8079";

fn main() {
    let sc = SocketConnection::new(PEER_ADDR, SERVER_ADDR, false).expect(SError::msg(SErrorKind::SocketConnection));
    let mut client = PClient { state: 42, ..Default::default() };
    let server = PServer::new();
    loop {
        println!("sending...");
        client.lookat += 1;
        sc.send(client).unwrap();
        sc.send(server).unwrap();
        thread::sleep(time::Duration::from_millis(500)); // frame_time. OW netcode is based on quantized 16ms, 7ms if tournament 
    };
}
// BE CAREFUL, if `datagram` is too short -> crash of bincode