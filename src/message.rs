use crate::game::board::Position;
use crate::game::common::{Color, GameResult};

// client
pub fn open_message(name: &str) -> String {
    vec!["OPEN", name].join(" ")
}

pub fn move_message(pos: Position) -> String {
    vec!["MOVE", &pos.to_cell_string()].join(" ")
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
        pos: Option<Position>,
    },
    Ack {
        remaining_time_ms: u32,
    },
    Bye {
        stat: String,
    },
}
