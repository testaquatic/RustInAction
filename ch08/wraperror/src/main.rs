use core::net;
use std::{error, fmt, fs::File, io, net::Ipv6Addr};

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl error::Error for UpstreamError {}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpstreamError::IO(e) => write!(f, "IO error: {}", e),
            UpstreamError::Parsing(e) => write!(f, "Parsing error: {}", e),
        }
    }
}

impl From<io::Error> for UpstreamError {
    fn from(e: io::Error) -> Self {
        UpstreamError::IO(e)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(e: net::AddrParseError) -> Self {
        UpstreamError::Parsing(e)
    }
}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("nonexistent.txt")?;

    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
