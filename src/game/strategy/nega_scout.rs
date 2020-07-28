use std::cmp;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct NegaScout {
    pub should_stop: AtomicBool,
}

impl Strategy for NegaScout {
    fn next_move(&self, board: Board, color: Color, remaining_time_ms: i32) -> Option<Square> {
        let flippables = board.flippable_squares(color);
        if flippables == 0 {
            return None;
        }
        let count = flippables.count_ones();
        let depth = if count < 4 {
            8
        } else if count < 8 {
            6
        } else {
            4
        };

        let mut ret = None;
        let mut cur_max = -5000;
        let opposite = color.opposite();

        // need atmost 30 secs to execute exhausive search at the end.
        let time_limit = Duration::from_millis(remaining_time_ms as u64 / 2);
        let now = Instant::now();
        for i in (0..64).filter(|&x| flippables & 1 << x != 0) {
            if now.elapsed() > time_limit {
                println!("Timeout! Aborting.");
                self.should_stop.store(true, Ordering::Relaxed);
            }
            let cur_square = Square::from_uint(i);
            let next_board = board.flip(cur_square, color);
            let score =
                -self.nega_scout(next_board, opposite, depth, -5000, 5000, time_limit / count);
            if cur_max < score {
                cur_max = score;
                ret = Some(cur_square);
            } else {
                ret = ret.or(Some(cur_square));
            }
        }
        ret
    }
}

impl NegaScout {
    fn nega_scout(
        &self,
        board: Board,
        color: Color,
        depth: u8,
        mut alpha: i16,
        beta: i16,
        time_limit: Duration,
    ) -> i16 {
        let flippables = board.flippable_squares(color);
        if depth == 0 || flippables == 0 || self.should_stop.load(Ordering::Relaxed) {
            return board.score(color);
        }

        let now = Instant::now();
        let opposite = color.opposite();
        let (first, rest) = Self::order_moves(board, color);
        let count = rest.len() as u32;
        let score = -self.nega_scout(
            board.flip(Square::from_uint(first), color),
            opposite,
            depth - 1,
            -beta,
            -alpha,
            time_limit / (count + 1) / 2,
        );
        alpha = cmp::max(alpha, score);
        for (i, mv) in rest.iter().enumerate() {
            if self.should_stop.load(Ordering::Relaxed) {
                break;
            }
            if now.elapsed() >= time_limit {
                println!("Timeout! Aborting. {:?} {:?}", time_limit, now.elapsed());
                self.should_stop.store(true, Ordering::Relaxed);
                break;
            }
            let cur_square = Square::from_uint(*mv);
            let next_board = board.flip(cur_square, color);
            let score = {
                let tmp_score = -self.nega_scout(
                    next_board,
                    opposite,
                    depth - 1,
                    -alpha - 1,
                    -alpha,
                    (time_limit - now.elapsed()) / (count - i as u32) / 2,
                );
                if now.elapsed() >= time_limit {
                    println!("Timeout! Aborting.");
                    self.should_stop.store(true, Ordering::Relaxed);
                    break;
                }
                if alpha < tmp_score && tmp_score < beta {
                    -self.nega_scout(
                        next_board,
                        opposite,
                        depth - 1,
                        -beta,
                        -tmp_score,
                        (time_limit - now.elapsed()) / (count - i as u32) / 2,
                    )
                } else {
                    tmp_score
                }
            };

            alpha = cmp::max(alpha, score);

            // beta cut-off
            if alpha >= beta {
                break;
            }
        }
        alpha
    }

    #[inline]
    fn order_moves(board: Board, color: Color) -> (u8, Vec<u8>) {
        let flippables = board.flippable_squares(color);
        let flippables = (0..64)
            .filter(|&s| flippables & 1 << s != 0)
            .collect::<Vec<u8>>();
        // using mini_nega_scout is maybe too slow?
        let max = flippables
            .iter()
            .cloned()
            .max_by_key(|&x| board.flip(Square::from_uint(x), color).score(color))
            .unwrap();
        (
            max,
            flippables.iter().cloned().filter(|&e| e != max).collect(),
        )
    }

    #[allow(dead_code)]
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
