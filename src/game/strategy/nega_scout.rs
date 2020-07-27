use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct NegaScout {
    pub depth: u8,
}

impl Strategy for NegaScout {
    fn next_move(&mut self, board: Board, color: Color) -> Option<Square> {
        let flippables = board.flippable_squares(color);
        if flippables == 0 {
            return None;
        }
        let mut ret = None;
        let mut cur_max = -100_f32;
        for i in 0..64 {
            if flippables & 1 << i != 0 {
                let cur_square = Square::from_uint(i);
                let next_board = board.flip(cur_square, color);
                let score =
                    -self.score(next_board, color.opposite(), self.depth, -100_f32, 100_f32);
                if cur_max < score {
                    cur_max = score;
                    ret = Some(cur_square);
                }
            }
        }
        ret
    }
}

impl NegaScout {
    // TODO: search killer hand first (ex. search for corners).
    fn score(&self, board: Board, color: Color, depth: u8, alpha: f32, beta: f32) -> f32 {
        let flippables = board.flippable_squares(color);
        if depth == 0 || flippables == 0 {
            return board.score(color);
        }
        let mut alpha = alpha;
        let mut is_first = true;

        for i in 0..64 {
            if flippables & 1 << i != 0 {
                let cur_square = Square::from_uint(i);
                let next_board = board.flip(cur_square, color);
                let opposite = color.opposite();
                let score = if is_first {
                    is_first = false;
                    let tmp_score =
                        -self.score(next_board, opposite, depth - 1, -alpha - 1_f32, -alpha);
                    if alpha < tmp_score && tmp_score < beta {
                        -self.score(next_board, opposite, depth - 1, -beta, -tmp_score)
                    } else {
                        tmp_score
                    }
                } else {
                    -self.score(next_board, opposite, depth - 1, -beta, -alpha)
                };
                if alpha < score {
                    alpha = score;
                }
                if alpha >= beta {
                    break;
                }
            }
        }
        alpha
    }
}
