use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;

pub mod exhausive;
pub mod naive;
pub mod nega_scout;
pub mod opening;

pub use exhausive::Exhausive;
pub use naive::Naive;
pub use nega_scout::NegaScout;
pub use opening::Opening;

pub trait Strategy {
    fn next_move(&self, board: Board, color: Color, remaining_time_ms: i32) -> Option<Square>;
}

impl dyn Strategy {
    pub fn default() -> Naive {
        Naive {}
    }
}
