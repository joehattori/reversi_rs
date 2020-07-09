use crate::game::common::{Color, GameResult};
use std::str::SplitWhitespace;

pub enum ServerMsg {
    Start {
        color: Color,
        op_name: String,
        remaining_time_ms: u16,
    },
    End {
        result: GameResult,
        player_count: u8,
        op_count: u8,
        reason: String,
    },
    Move {
        point: String,
    },
    Ack {
        remaining_time_ms: u16,
    },
    Bye {
        stat: String,
    },
}

pub fn parse<'a>(buf: String) -> Result<ServerMsg, &'a str> {
    let mut split = buf.split_whitespace();
    match split.next() {
        Some(cmd) => match cmd {
            "START" => return parse_start(&mut split),
            "END" => return parse_end(&mut split),
            "MOVE" => return parse_move(&mut split),
            "ACK" => return parse_move(&mut split),
            "BYE" => return parse_move(&mut split),
            _ => return Err("Invalid command"),
        },
        None => return Err("test"),
    }
}

fn parse_start<'a>(split: &mut SplitWhitespace) -> Result<ServerMsg, &'a str> {
    let color = match split.next() {
        Some(cmd) => match cmd {
            "BLACK" => Color::Black,
            "WHITE" => Color::White,
            _ => return Err("While parsing start: Invalid color."),
        },
        None => return Err("While parsing start: Invalid message."),
    };
    let op_name = match split.next() {
        Some(name) => name,
        None => return Err("While parsing start: Invalid message."),
    };
    let remaining_time = match split.next() {
        Some(time) => match time.parse() {
            Ok(i) => i,
            Err(_) => return Err("While parsing start: Invalid time."),
        },
        None => return Err("While parsing start: Invalid message."),
    };
    Ok(ServerMsg::Start {
        color: color,
        op_name: op_name.to_string(),
        remaining_time_ms: remaining_time,
    })
}

fn parse_end<'a>(split: &mut SplitWhitespace) -> Result<ServerMsg, &'a str> {
    let result = match split.next() {
        Some(r) => match r {
            "Win" => GameResult::Win,
            "Lose" => GameResult::Lose,
            "Tie" => GameResult::Tie,
            _ => return Err("While parsing end: Invalid result."),
        },
        None => return Err("While parsing end: Invalid message."),
    };
    let player_count = match split.next() {
        Some(n) => match n.parse() {
            Ok(i) => i,
            Err(_) => return Err("While parsing end: Invalid count."),
        },
        None => return Err("While parsing end: Invalid message."),
    };
    let op_count = match split.next() {
        Some(n) => match n.parse() {
            Ok(i) => i,
            Err(_) => return Err("While parsing end: Invalid count."),
        },
        None => return Err("While parsing end: Invalid message."),
    };
    let reason = match split.next() {
        Some(s) => s,
        None => return Err("While parsing end: Invalid message."),
    }
    .to_string();
    Ok(ServerMsg::End {
        result: result,
        player_count: player_count,
        op_count: op_count,
        reason: reason,
    })
}

fn parse_move<'a>(split: &mut SplitWhitespace) -> Result<ServerMsg, &'a str> {
    let point = match split.next() {
        Some(s) => s,
        None => return Err("While parsing move: Invalid message."),
    }
    .to_string();
    Ok(ServerMsg::Move { point: point })
}

fn parse_ack<'a>(split: &mut SplitWhitespace) -> Result<ServerMsg, &'a str> {
    let remaining_time_ms = match split.next() {
        Some(s) => match s.parse() {
            Ok(i) => i,
            Err(_) => return Err("While parsing ack: Invalid time."),
        },
        None => return Err("While parsing ack: Invalid message."),
    };
    Ok(ServerMsg::Ack {
        remaining_time_ms: remaining_time_ms,
    })
}

fn parse_bye<'a>(split: &mut SplitWhitespace) -> Result<ServerMsg, &'a str> {
    let stat = match split.next() {
        Some(s) => s,
        None => return Err("While parsing bye: Invalid message."),
    }
    .to_string();
    Ok(ServerMsg::Bye { stat: stat })
}
