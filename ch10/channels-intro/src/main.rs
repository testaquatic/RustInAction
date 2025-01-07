use std::thread;

use crossbeam::{channel::unbounded, select};

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

fn main() {
    let n_messages = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            ConnectivityCheck::Ping => responses_tx.send(ConnectivityCheck::Pong).unwrap(),
            ConnectivityCheck::Pong => eprintln!("unexpected pong response"),
            ConnectivityCheck::Pang => return,
        }
    });

    for _ in 0..n_messages {
        requests_tx.send(ConnectivityCheck::Ping).unwrap();
    }
    requests_tx.send(ConnectivityCheck::Pang).unwrap();

    for _ in 0..n_messages {
        select! {
            recv(responses_rx) -> msg => println!("{:?}", msg),
        }
    }
}
