use crate::game::base::{Color, GameResult};
use crate::game::square::Square;

// client
pub fn open_message(name: &str) -> String {
    vec!["OPEN", name].join(" ")
}

pub fn move_message(s: Square) -> String {
    vec!["MOVE", &s.to_string()].join(" ")
}

pub fn pass_message() -> String {
    "MOVE PASS".to_string()
}

// server
pub enum ServerMessage {
    Start {
        color: Color,
        op_name: String,
        remaining_time_ms: u32,
    },
    End {
        result: GameResult,
        player_count: u8,
        op_count: u8,
        reason: String,
    },
    Move {
        pos: Option<Square>,
    },
    Ack {
        remaining_time_ms: u32,
    },
    Bye {
        stat: String,
    },
}
