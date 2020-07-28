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
        let mut cur_max = -5000;
        let opposite = color.opposite();
        for i in 0..64 {
            if flippables & 1 << i != 0 {
                let cur_square = Square::from_uint(i);
                let next_board = board.flip(cur_square, color);
                let score = -Self::nega_scout(next_board, opposite, self.depth - 1, -5000, 5000);
                if cur_max < score {
                    cur_max = score;
                    ret = Some(cur_square);
                } else if ret.is_none() {
                    ret = Some(cur_square);
                }
            }
        }
        ret
    }
}

impl NegaScout {
    fn nega_scout(board: Board, color: Color, depth: u8, mut alpha: i16, beta: i16) -> i16 {
        let flippables = board.flippable_squares(color);
        if depth == 0 || flippables == 0 {
            return board.score(color);
        }
        let mut is_first = true;

        for i in Self::order_moves(board, color) {
            let cur_square = Square::from_uint(i);
            let next_board = board.flip(cur_square, color);
            let opposite = color.opposite();
            let score = if is_first {
                is_first = false;
                -Self::mini_nega_scout(next_board, opposite, depth - 1, -beta, -alpha)
            } else {
                let tmp_score =
                    -Self::mini_nega_scout(next_board, opposite, depth - 1, -alpha - 1, -alpha);
                if alpha < tmp_score && tmp_score < beta {
                    -Self::mini_nega_scout(next_board, opposite, depth - 1, -beta, -tmp_score)
                } else {
                    tmp_score
                }
            };
            if alpha < score {
                alpha = score;
            }
            // beta cut-off
            if alpha >= beta {
                break;
            }
        }
        alpha
    }

    #[inline]
    fn order_moves(board: Board, color: Color) -> Vec<u8> {
        let flippables = board.flippable_squares(color);
        let mut flippables = (0..64)
            .filter(|&s| flippables & 1 << s != 0)
            .collect::<Vec<u8>>();
        // using mini_nega_scout is maybe too slow?
        //let opposite = color.opposite();
        flippables.sort_by(|a, b| {
            let a_score = board.flip(Square::from_uint(*b), color).score(color);
            let b_score = board.flip(Square::from_uint(*a), color).score(color);
            b_score.partial_cmp(&a_score).unwrap()
            //let next_a = -Self::mini_nega_scout(
            //board.flip(Square::from_uint(*a), color),
            //opposite,
            //1,
            //-5000,
            //100_i16,
            //);
            //let next_b = -Self::mini_nega_scout(
            //board.flip(Square::from_uint(*b), color),
            //opposite,
            //1,
            //-5000,
            //5000,
            //);
            //next_b.partial_cmp(&next_a).unwrap()
        });
        flippables
    }

    fn mini_nega_scout(board: Board, color: Color, depth: u8, mut alpha: i16, beta: i16) -> i16 {
        let flippables = board.flippable_squares(color);
        if depth == 0 || flippables == 0 {
            return board.score(color);
        }
        let mut is_first = true;
        for i in (0..64).filter(|&s| flippables & 1 << s != 0) {
            let cur_square = Square::from_uint(i);
            let next_board = board.flip(cur_square, color);
            let opposite = color.opposite();
            let score = if is_first {
                is_first = false;
                -Self::mini_nega_scout(next_board, opposite, depth - 1, -beta, -alpha)
            } else {
                let tmp_score =
                    -Self::mini_nega_scout(next_board, opposite, depth - 1, -alpha - 1_i16, -alpha);
                if alpha < tmp_score && tmp_score < beta {
                    -Self::mini_nega_scout(next_board, opposite, depth - 1, -beta, -tmp_score)
                } else {
                    tmp_score
                }
            };
            if alpha < score {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        }
        alpha
    }
}
