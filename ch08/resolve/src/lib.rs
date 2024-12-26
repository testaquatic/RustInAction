use clap::{Arg, Command};

#[derive(Debug)]
pub struct Args {
    pub dns_server: String,
    pub domain_name: String,
}

pub fn get_args() -> Args {
    let matches = Command::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(Arg::new("dns_server").short('s').default_value("1.1.1.1"))
        .arg(Arg::new("domain_name").required(true))
        .get_matches();

    Args {
        dns_server: matches.get_one::<String>("dns_server").unwrap().clone(),
        domain_name: matches.get_one::<String>("domain_name").unwrap().clone(),
    }
}
