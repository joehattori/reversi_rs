use crate::game::board::Position;
use crate::game::common::{Color, GameResult};
use crate::message::ServerMessage;
use std::io;
use std::io::{BufRead, Write};
use std::net;
use std::str::SplitWhitespace;

pub struct Client {
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
                        reader: r,
                        writer: w,
                    }
                }
                Err(_) => panic!("Couldn't clone stream"),
            },
            Err(_) => panic!("Couldn't connect to {}:{}", host, port),
        }
    }

    pub fn poll_message(&mut self) -> Result<ServerMessage, String> {
        loop {
            let mut buf = String::new();
            match self.reader.read_line(&mut buf) {
                Ok(_) => (),
                Err(_) => return Err("Error occured while reading message.".to_string()),
            }
            if buf.len() == 0 {
                continue;
            }
            println!("Read: {}", buf);
            return self.parse_input(buf);
        }
    }

    pub fn send_message(&mut self, msg: &str) -> Result<(), &str> {
        match writeln!(self.writer, "{}", msg) {
            Ok(_) => {
                self.writer.flush().unwrap();
                println!("Sent {}", msg);
                Ok(())
            }
            _ => Err("Failed to send"),
        }
    }
    pub fn parse_input(&self, buf: String) -> Result<ServerMessage, String> {
        let mut split = buf.split_whitespace();
        match split.next() {
            Some(cmd) => match cmd {
                "START" => return self.parse_start(&mut split),
                "END" => return self.parse_end(&mut split),
                "MOVE" => return self.parse_move(&mut split),
                "ACK" => return self.parse_ack(&mut split),
                "BYE" => return self.parse_bye(&mut split),
                _ => return Err("Invalid command".to_string()),
            },
            None => return Err("test".to_string()),
        }
    }

    fn parse_start(&self, split: &mut SplitWhitespace) -> Result<ServerMessage, String> {
        let color = match split.next() {
            Some(cmd) => match cmd {
                "BLACK" => Color::Black,
                "WHITE" => Color::White,
                _ => return Err("While parsing start: Invalid color.".to_string()),
            },
            None => return Err("While parsing start: Invalid message.".to_string()),
        };
        let op_name = match split.next() {
            Some(name) => name,
            None => return Err("While parsing start: Invalid message.".to_string()),
        }
        .to_string();
        let remaining_time_ms = match split.next() {
            Some(time) => match time.parse() {
                Ok(i) => i,
                Err(s) => {
                    return Err(format!("While parsing start: Invalid time: {}", s).to_string())
                }
            },
            None => return Err("While parsing start: Invalid message.".to_string()),
        };
        Ok(ServerMessage::Start {
            color,
            op_name,
            remaining_time_ms,
        })
    }

    fn parse_end(&self, split: &mut SplitWhitespace) -> Result<ServerMessage, String> {
        let result = match split.next() {
            Some(r) => match r {
                "WIN" => GameResult::Win,
                "LOSE" => GameResult::Lose,
                "TIE" => GameResult::Tie,
                _ => return Err("While parsing end: Invalid result.".to_string()),
            },
            None => return Err("While parsing end: Invalid message.".to_string()),
        };
        let player_count = match split.next() {
            Some(n) => match n.parse() {
                Ok(i) => i,
                Err(_) => return Err("While parsing end: Invalid count.".to_string()),
            },
            None => return Err("While parsing end: Invalid message.".to_string()),
        };
        let op_count = match split.next() {
            Some(n) => match n.parse() {
                Ok(i) => i,
                Err(_) => return Err("While parsing end: Invalid count.".to_string()),
            },
            None => return Err("While parsing end: Invalid message.".to_string()),
        };
        let reason = match split.next() {
            Some(s) => s,
            None => return Err("While parsing end: Invalid message.".to_string()),
        }
        .to_string();
        Ok(ServerMessage::End {
            result,
            player_count,
            op_count,
            reason,
        })
    }

    fn parse_move(&self, split: &mut SplitWhitespace) -> Result<ServerMessage, String> {
        let pos = match split.next() {
            Some(s) => s,
            None => return Err("While parsing move: Invalid message.".to_string()),
        };
        let pos = if pos == "PASS" {
            None
        } else {
            match Position::from_str(pos) {
                Ok(p) => Some(p),
                Err(s) => return Err(s.to_string()),
            }
        };
        Ok(ServerMessage::Move { pos })
    }

    fn parse_ack(&self, split: &mut SplitWhitespace) -> Result<ServerMessage, String> {
        let remaining_time_ms = match split.next() {
            Some(s) => match s.parse() {
                Ok(i) => i,
                Err(_) => return Err("While parsing ack: Invalid time.".to_string()),
            },
            None => return Err("While parsing ack: Invalid message.".to_string()),
        };
        Ok(ServerMessage::Ack { remaining_time_ms })
    }

    fn parse_bye(&self, split: &mut SplitWhitespace) -> Result<ServerMessage, String> {
        let stat = match split.next() {
            Some(s) => s,
            None => return Err("While parsing bye: Invalid message.".to_string()),
        }
        .to_string();
        Ok(ServerMessage::Bye { stat })
    }
}
