use core::{error, fmt};
use std::net::IpAddr;

use smoltcp::{
    iface::{self, Config, Interface, PollResult, RouteTableFull, SocketSet},
    phy::TunTapInterface,
    socket::tcp::{self, SocketBuffer},
    time::Instant,
    wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address},
};
use url::Url;

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response,
}

#[derive(Debug)]
pub enum UpstreamError {
    Network(smoltcp::wire::Error),
    InvaliUrl,
    Content(std::str::Utf8Error),
    RouteTableFull(iface::RouteTableFull),
    ConnetError(tcp::ConnectError),
    SendError(tcp::SendError),
    RecvError(tcp::RecvError),
}

impl error::Error for UpstreamError {}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<smoltcp::wire::Error> for UpstreamError {
    fn from(e: smoltcp::wire::Error) -> Self {
        UpstreamError::Network(e)
    }
}

impl From<std::str::Utf8Error> for UpstreamError {
    fn from(e: std::str::Utf8Error) -> Self {
        UpstreamError::Content(e)
    }
}

impl From<RouteTableFull> for UpstreamError {
    fn from(e: RouteTableFull) -> Self {
        UpstreamError::RouteTableFull(e)
    }
}

impl From<tcp::ConnectError> for UpstreamError {
    fn from(e: tcp::ConnectError) -> Self {
        UpstreamError::ConnetError(e)
    }
}

impl From<tcp::SendError> for UpstreamError {
    fn from(e: tcp::SendError) -> Self {
        UpstreamError::SendError(e)
    }
}

impl From<tcp::RecvError> for UpstreamError {
    fn from(e: tcp::RecvError) -> Self {
        UpstreamError::RecvError(e)
    }
}

fn random_port() -> u16 {
    49152 + rand::random::<u16>() % 16384
}

pub fn get(
    mut tap: TunTapInterface,
    mac: EthernetAddress,
    addr: IpAddr,
    url: Url,
) -> Result<(), UpstreamError> {
    let config = Config::new(mac.into());
    let mut iface = Interface::new(config, &mut tap, Instant::now());

    let default_gateway = Ipv4Address::new(192, 168, 42, 100);
    iface.routes_mut().add_default_ipv4_route(default_gateway)?;

    let ip_addrs: IpCidr = IpCidr::new(IpAddress::v4(192, 168, 42, 1), 24);
    iface.update_ip_addrs(|inner| inner.push(ip_addrs).unwrap());

    let mut sockets = SocketSet::new(vec![]);
    let tcp_rx_buffer = SocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = SocketBuffer::new(vec![0; 1024]);
    let tcp_socket = tcp::Socket::new(tcp_rx_buffer, tcp_tx_buffer);
    let tcp_handle = sockets.add(tcp_socket);

    let domain_name = url.host_str().ok_or(UpstreamError::InvaliUrl)?;
    let http_header = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path(),
        domain_name
    );

    let mut state = HttpState::Connect;
    'http: loop {
        let timestamp = Instant::now();
        match iface.poll(timestamp, &mut tap, &mut sockets) {
            PollResult::None => (),
            e => eprintln!("{e:?}"),
        }

        let socket = sockets.get_mut::<tcp::Socket>(tcp_handle);

        state = match state {
            HttpState::Connect if !socket.is_active() => {
                eprintln!("connecting");
                socket.connect(iface.context(), (addr, 80), random_port())?;
                HttpState::Request
            }

            HttpState::Request if socket.may_send() => {
                eprintln!("sending requested");
                socket.send_slice(http_header.as_bytes())?;
                HttpState::Response
            }

            HttpState::Response if socket.can_recv() => {
                socket.recv(|raw_data| {
                    let output = String::from_utf8_lossy(raw_data);
                    println!("{}", output);
                    (raw_data.len(), ())
                })?;
                HttpState::Response
            }

            HttpState::Response if !socket.may_recv() => {
                eprintln!("received complete response");
                break 'http;
            }

            _ => state,
        }
    }

    Ok(())
}
