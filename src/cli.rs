use crate::game::common::{Color, GameResult};
use crate::game::square::Square;
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
    pub fn new(host: &str, port: u32) -> Self {
        let addr = host.to_string() + ":" + &port.to_string();
        let r_stream =
            net::TcpStream::connect(addr).expect(&format!("Couldn't connect to {}:{}", host, port));
        let w_stream = r_stream.try_clone().expect("Couldn't clone stream.");
        let w = io::BufWriter::new(w_stream);
        let r = io::BufReader::new(r_stream);
        Self {
            reader: r,
            writer: w,
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

    pub fn send_message(&mut self, msg: &str) -> Result<(), String> {
        writeln!(self.writer, "{}", msg)
            .map_err(|_| "Couldn't send.".to_string())
            .map(|_| {
                self.writer.flush().unwrap();
                println!("Sent {}", msg);
            })
    }
    pub fn parse_input(&self, buf: String) -> Result<ServerMessage, String> {
        let mut split = buf.split_whitespace();
        match split.next() {
            Some(cmd) => match cmd {
                "START" => self.parse_start(&mut split),
                "END" => self.parse_end(&mut split),
                "MOVE" => self.parse_move(&mut split),
                "ACK" => self.parse_ack(&mut split),
                "BYE" => self.parse_bye(&mut split),
                _ => Err("Invalid command".to_string()),
            },
            None => Err("test".to_string()),
        }
    }

    fn parse_start(&self, split: &mut SplitWhitespace) -> Result<ServerMessage, String> {
        let color = match split.next() {
            Some(cmd) => match cmd {
                "BLACK" => Color::Dark,
                "WHITE" => Color::Light,
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
            match Square::from_str(pos) {
                Ok(p) => Some(p),
                Err(s) => return Err(s.to_string()),
            }
        };
        Ok(ServerMessage::Move { pos })
    }

    fn parse_ack(&self, split: &mut SplitWhitespace) -> Result<ServerMessage, String> {
        split
            .next()
            .ok_or("While parsing ack: Invalid message.".to_string())
            .and_then(|s| {
                s.parse()
                    .map_err(|_| "While parsing ack: Invalid time.".to_string())
            })
            .map(|remaining_time_ms| ServerMessage::Ack { remaining_time_ms })
    }

    fn parse_bye(&self, split: &mut SplitWhitespace) -> Result<ServerMessage, String> {
        split
            .next()
            .ok_or("While parsing bye: Invalid message.".to_string())
            .map(|s| ServerMessage::Bye {
                stat: s.to_string(),
            })
    }
}
