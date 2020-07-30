use std::time::{Duration, Instant};

use super::{Naive, NegaScout};
use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::opening_db::{DARK_MOVES, LIGHT_MOVES};
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct Opening {
    pub now: Instant,
    pub time_limit: Duration,
}

impl Strategy for Opening {
    fn next_move(&self, board: Board, color: Color) -> Option<Square> {
        let moves = match color {
            Color::Dark => DARK_MOVES.lock().unwrap(),
            Color::Light => LIGHT_MOVES.lock().unwrap(),
        };
        moves
            .get(&board)
            .map(|s| *s)
            .or(self.switch_to_nega_scout(board, color))
    }
}

impl Opening {
    pub fn new(time_limit_millisec: u64) -> Self {
        Self {
            now: Instant::now(),
            time_limit: Duration::from_millis(time_limit_millisec),
        }
    }

    fn switch_to_nega_scout(&self, board: Board, color: Color) -> Option<Square> {
        let em = Naive().next_move(board, color);
        let rest = self
            .time_limit
            .checked_sub(self.now.elapsed().div_f32(4_f32))
            .unwrap_or(Duration::new(0, 0));
        NegaScout::new_from_duration(rest, em).next_move(board, color)
    }
}
