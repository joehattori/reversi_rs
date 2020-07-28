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
        let now = Instant::now();
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
        let time_limit = Duration::from_millis(remaining_time_ms as u64 * 3 / 8);
        for (i, mv) in (0..64).filter(|&x| flippables & 1 << x != 0).enumerate() {
            let remaining = match time_limit.checked_sub(now.elapsed()) {
                Some(t) => t,
                None => {
                    println!("Timeout! Aborting.");
                    self.should_stop.store(true, Ordering::Relaxed);
                    Duration::new(0, 0)
                }
            };
            let next_board = board.flip(mv, color);
            let score = -self.nega_scout(
                next_board,
                opposite,
                depth,
                -5000,
                5000,
                remaining / (count - i as u32),
            );
            let cur_square = Square::from_uint(mv);
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
        let now = Instant::now();
        let flippables = board.flippable_squares(color);
        if depth == 0 || flippables == 0 || self.should_stop.load(Ordering::Relaxed) {
            return board.score(color);
        }

        let opposite = color.opposite();
        let (first, rest) = Self::order_moves(board, color);
        let count = rest.len() as u32;
        let remaining = match time_limit.checked_sub(now.elapsed()) {
            Some(t) => t,
            None => {
                println!("Timeout! Aborting.");
                return board.score(color);
            }
        };
        let score = -self.nega_scout(
            board.flip(first, color),
            opposite,
            depth - 1,
            -beta,
            -alpha,
            remaining / (count + 1) / 2,
        );
        alpha = cmp::max(alpha, score);
        for (i, mv) in rest.iter().enumerate() {
            if self.should_stop.load(Ordering::Relaxed) {
                break;
            }
            let next_board = board.flip(*mv, color);
            let remaining = match time_limit.checked_sub(now.elapsed()) {
                Some(t) => t,
                None => {
                    self.should_stop.store(true, Ordering::Relaxed);
                    break;
                }
            };
            let score = {
                let tmp_score = -self.nega_scout(
                    next_board,
                    opposite,
                    depth - 1,
                    -alpha - 1,
                    -alpha,
                    remaining / (count - i as u32) / 2,
                );
                let remaining = match time_limit.checked_sub(now.elapsed()) {
                    Some(t) => t,
                    None => {
                        println!("Timeout! Aborting.");
                        break;
                    }
                };
                if alpha < tmp_score && tmp_score < beta {
                    -self.nega_scout(
                        next_board,
                        opposite,
                        depth - 1,
                        -beta,
                        -tmp_score,
                        remaining / (count - i as u32) / 2,
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
            .max_by_key(|&x| board.flip(x, color).score(color))
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
        for square in (0..64).filter(|&s| flippables & 1 << s != 0) {
            let next_board = board.flip(square, color);
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
