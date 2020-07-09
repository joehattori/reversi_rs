use std::io;
use std::net;

pub struct Client<'a> {
    host: &'a str,
    port: i32,
    reader: io::BufReader<net::TcpStream>,
    writer: io::BufWriter<net::TcpStream>,
}

impl<'a> Client<'a> {
    pub fn new(host: &str, port: i32) -> Client {
        let addr = host.to_string() + ":" + &port.to_string();
        match net::TcpStream::connect(addr) {
            Ok(stream) => match stream.try_clone() {
                Ok(s) => {
                    let w = io::BufWriter::new(s);
                    let r = io::BufReader::new(stream);
                    Client {
                        host: host,
                        port: port,
                        reader: r,
                        writer: w,
                    }
                }
                Err(_) => panic!("Couldn't clone stream"),
            },
            Err(_) => panic!("Couldn't connect to {}:{}", host, port),
        }
    }
}
