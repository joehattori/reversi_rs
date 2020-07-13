use crate::game::board::Board;
use crate::game::common::Color;
use crate::game::square::Square;

pub mod exhausive;
pub mod naive;

pub use exhausive::Exhausive;
pub use naive::Naive;

pub trait Strategy {
    fn next_move(&mut self, board: &Board, color: Color) -> Option<Square>;
}

impl dyn Strategy {
    pub fn default() -> Naive {
        Naive {}
    }
}
