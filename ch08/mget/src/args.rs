use clap::{Arg, Command};
use smoltcp::phy::{Medium, TunTapInterface};
use url::{ParseError, Url};

use crate::errors::MgetError;

#[derive(Debug)]
pub struct CommandArgs {
    url: String,
    tap_device: String,
    dns_server: String,
}

impl CommandArgs {
    pub fn get() -> Result<CommandArgs, MgetError> {
        let matches = Command::new("mget")
            .about("GET a webpage, manually")
            .arg(
                Arg::new("url")
                    .help("The URL to GET")
                    .required(true)
                    .num_args(1),
            )
            .arg(
                Arg::new("tap_device")
                    .help("The tap device to use")
                    .required(true)
                    .num_args(1),
            )
            .arg(
                Arg::new("dns-server")
                    .help("The DNS server to use")
                    .default_value("1.1.1.1"),
            )
            .get_matches();

        let args = CommandArgs {
            url: matches.get_one::<String>("url").unwrap().to_string(),
            tap_device: matches.get_one::<String>("tap_device").unwrap().to_string(),
            dns_server: matches.get_one::<String>("dns-server").unwrap().to_string(),
        };

        Ok(args)
    }

    pub fn url(&self) -> Result<Url, ParseError> {
        Url::parse(&self.url)
    }

    pub fn tab_interface(&self) -> Result<TunTapInterface, std::io::Error> {
        TunTapInterface::new(&self.tap_device, Medium::Ethernet)
    }

    pub fn dns_server_addr(&self) -> Result<std::net::Ipv4Addr, std::net::AddrParseError> {
        self.dns_server.parse()
    }
}
