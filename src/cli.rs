use crate::message::server::{parse, ServerMsg};
use std::io;
use std::io::BufRead;
use std::net;

pub struct Client {
    host: String,
    port: u32,
    reader: io::BufReader<net::TcpStream>,
    writer: io::BufWriter<net::TcpStream>,
}

impl Client {
    pub fn new(host: &str, port: u32) -> Client {
        let addr = host.to_string() + ":" + &port.to_string();
        match net::TcpStream::connect(addr) {
            Ok(stream) => match stream.try_clone() {
                Ok(s) => {
                    let w = io::BufWriter::new(s);
                    let r = io::BufReader::new(stream);
                    Client {
                        host: host.to_string(),
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

    pub fn read_message(&mut self) -> Result<ServerMsg, &str> {
        loop {
            let mut buf = String::new();
            match self.reader.read_line(&mut buf) {
                Ok(_) => (),
                Err(_) => return Err("Error occured while reading message."),
            }
            if buf.len() == 0 {
                continue;
            }
            println!("read: {}", buf);
            return parse(buf);
        }
    }
}
