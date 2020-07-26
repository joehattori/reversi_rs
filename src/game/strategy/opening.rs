use crate::game::board::Board;
use crate::game::common::Color;
use crate::game::opening_db::{DARK_MOVES, LIGHT_MOVES};
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct Opening {}

impl Strategy for Opening {
    fn next_move(&mut self, board: Board, color: Color) -> Option<Square> {
        let moves = match color {
            Color::Dark => &*DARK_MOVES,
            Color::Light => &*LIGHT_MOVES,
        };
        moves.get(&board).map(|s| *s).or(naive(board, color))
    }
}

fn naive(board: Board, color: Color) -> Option<Square> {
    let flippables = board.flippable_squares(color);
    for i in 0..64 {
        if flippables & 1 << i != 0 {
            return Some(Square::from_uint(i));
        }
    }
    None
}
