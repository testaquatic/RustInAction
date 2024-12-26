use std::{
    io::{self, Write},
    net::TcpStream,
};

fn main() -> Result<(), io::Error> {
    let host = "www.rustinaction.com:80";

    let mut connection = TcpStream::connect(host)?;

    connection.write_all(b"GET / HTTP/1.1\r\n")?;
    connection.write_all(b"Host: www.rustinaction.com\r\n\r\n")?;

    std::io::copy(&mut connection, &mut std::io::stdout())?;

    Ok(())
}
