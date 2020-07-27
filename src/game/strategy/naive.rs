use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct Naive {}

impl Strategy for Naive {
    fn next_move(&mut self, board: Board, color: Color) -> Option<Square> {
        let flippables = board.flippable_squares(color);
        for i in 0..64 {
            if flippables & 1 << i != 0 {
                return Some(Square::from_uint(i));
            }
        }
        None
    }
}
