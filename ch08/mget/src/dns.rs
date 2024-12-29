use core::{error, fmt};
use std::{net::SocketAddr, time::Duration};

use hickory_client::{
    op::{Message, MessageType, OpCode, Query},
    proto::error::ProtoError,
    rr::{Name, RecordType},
    serialize::binary::{BinEncodable, BinEncoder},
};
use tokio::{net::UdpSocket, time::timeout};

fn message_id() -> u16 {
    loop {
        let candidate = rand::random::<u16>();
        if candidate != 0 {
            return candidate;
        }
    }
}

#[derive(Debug)]
pub enum DnsError {
    ParseDomainName(ProtoError),
    ParseDnsServerAddress(std::net::AddrParseError),
    Encoding(ProtoError),
    Decoding(ProtoError),
    Network(std::io::Error),
    Sending(std::io::Error),
    Receving(std::io::Error),
    Timeout(tokio::time::error::Elapsed),
}

impl error::Error for DnsError {}

impl fmt::Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub async fn resolve(
    dns_server_address: &str,
    domain_name: &str,
) -> Result<Option<std::net::IpAddr>, DnsError> {
    let domain_name = Name::from_ascii(domain_name).map_err(DnsError::ParseDomainName)?;

    let dns_server: SocketAddr = format!("{}:53", dns_server_address)
        .parse()
        .map_err(DnsError::ParseDnsServerAddress)?;

    let mut request_buffer = Vec::with_capacity(64);
    let mut response_buffer = vec![0_u8; 512];

    let mut request = Message::new();
    let query = Query::query(domain_name, RecordType::A);

    request
        .add_query(query)
        .set_id(message_id())
        .set_message_type(MessageType::Query)
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let localhost = UdpSocket::bind("0.0.0.0:0")
        .await
        .map_err(DnsError::Network)?;

    let out = Duration::from_secs(5);

    let mut encoder = BinEncoder::new(&mut request_buffer);
    request.emit(&mut encoder).map_err(DnsError::Encoding)?;

    let _n_bytes_sent = timeout(out, localhost.send_to(&request_buffer, dns_server))
        .await
        .map_err(DnsError::Timeout)?
        .map_err(DnsError::Sending)?;

    loop {
        let (_b_byte_recv, remote_port) = timeout(out, localhost.recv_from(&mut response_buffer))
            .await
            .map_err(DnsError::Timeout)?
            .map_err(DnsError::Receving)?;

        if remote_port == dns_server {
            break;
        }
    }

    let response = Message::from_vec(&response_buffer).map_err(DnsError::Decoding)?;

    let resolve = response
        .answers()
        .iter()
        .find(|answer| answer.record_type() == RecordType::A)
        .map(|answer| {
            let resource = answer.data().unwrap();

            resource.ip_addr().expect("Invalid IP address received")
        });

    Ok(resolve)
}
