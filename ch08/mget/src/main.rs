use libmget::{args::CommandArgs, dns, errors::MgetError, ethernet, http};

#[tokio::main]
async fn main() -> Result<(), MgetError> {
    let args = CommandArgs::get()?;

    #[cfg(debug_assertions)]
    println!("{:?}", args);

    let url = args.url()?;
    if url.scheme() != "http" {
        panic!("error: only HTTP protocol supported");
    }
    let domain_name = url.host_str().expect("domain anme required");
    let tap = args
        .tab_interface()
        .expect("error: unable to use <tap-device> as a network interface");
    let dns_server = args
        .dns_server_addr()
        .expect("error: unable to parse <dns_server> as an IPv4 address");
    let addr = dns::resolve(&dns_server.to_string(), domain_name)
        .await?
        .unwrap();

    let mac = ethernet::MacAddress::new().into();

    http::get(tap, mac, addr, url)?;

    Ok(())
}
