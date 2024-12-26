use std::net::SocketAddr;
use std::time::Duration;

use hickory_client::op::{Message, MessageType, OpCode, Query};
use hickory_client::rr::{Name, RecordType};
use libresolve::get_args;
use tokio::net::UdpSocket;
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let args = get_args();

    #[cfg(debug_assertions)]
    println!("{:?}", args);

    let domain_name = Name::from_ascii(&args.domain_name).expect("Invalid domain anme");

    let dns_server = format!("{}:53", args.dns_server)
        .parse::<SocketAddr>()
        .expect("Failed to parse dns server address.");

    let mut msg = Message::new();
    msg.set_id(rand::random())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);
    let request_as_byte = msg.to_vec().expect("Failed to encode message");

    let localhost = UdpSocket::bind("0.0.0.0:0")
        .await
        .expect("Failed to bind to local socket");

    localhost
        .send_to(&request_as_byte, dns_server)
        .await
        .expect("Socket missconfigured");

    let mut response_as_bytes = vec![0; 512];

    let _ = timeout(
        Duration::from_secs(3),
        localhost.recv_from(&mut response_as_bytes),
    )
    .await
    .expect("timeout reached");

    let dns_message = Message::from_vec(&response_as_bytes).expect("unable to parse response");

    dns_message
        .answers()
        .iter()
        .filter(|answer| answer.record_type() == RecordType::A)
        .for_each(|answer| {
            if let Some(data) = answer.data() {
                let ip = data.ip_addr().expect("Invalid IP address received");
                println!("{}", ip);
            }
        });
}
