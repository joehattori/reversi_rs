use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;

pub mod exhausive;
pub mod naive;
pub mod nega_scout;

pub use exhausive::Exhausive;
pub use naive::Naive;
pub use nega_scout::NegaScout;

pub trait Strategy {
    fn next_move(&self, board: Board, color: Color) -> Option<Square>;
}
