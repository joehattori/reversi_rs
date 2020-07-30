use std::cmp;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct NegaScout {
    pub should_stop: AtomicBool,
    pub now: Instant,
    pub time_limit: Duration,
    pub emergency_ret: Option<Square>,
}

impl Strategy for NegaScout {
    fn next_move(&self, board: Board, color: Color) -> Option<Square> {
        let flippables = board.flippable_squares(color);
        if flippables == 0 {
            return None;
        }
        let count = flippables.count_ones();
        // TODO: polish
        // TODO: think of using db
        let depth = if count < 4 {
            8
        } else if count < 8 {
            7
        } else {
            4
        };

        let mut ret = self.emergency_ret;
        let mut cur_max = -5000;
        let opposite = color.opposite();

        for mv in (0..64).filter(|&x| flippables & 1 << x != 0) {
            if self.should_stop.load(Ordering::Relaxed) {
                break;
            }
            self.check_time_limit();
            let next_board = board.flip(mv, color);
            let score = -self.nega_scout(next_board, opposite, depth, -5000, 5000);
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
    pub fn new(time_limit_millisec: u64, emergency_ret: Option<Square>) -> Self {
        Self {
            should_stop: AtomicBool::new(false),
            now: Instant::now(),
            time_limit: Duration::from_millis(time_limit_millisec),
            emergency_ret,
        }
    }

    pub fn new_from_duration(duration: Duration, emergency_ret: Option<Square>) -> Self {
        Self {
            should_stop: AtomicBool::new(false),
            now: Instant::now(),
            time_limit: duration,
            emergency_ret,
        }
    }

    fn nega_scout(&self, board: Board, color: Color, depth: u8, mut alpha: i16, beta: i16) -> i16 {
        let flippables = board.flippable_squares(color);
        if depth == 0 || flippables == 0 || self.should_stop.load(Ordering::Relaxed) {
            return board.score(color);
        }

        let opposite = color.opposite();
        let (first, rest) = Self::order_moves(board, color);
        let score = -self.nega_scout(board.flip(first, color), opposite, depth - 1, -beta, -alpha);
        self.check_time_limit();
        if self.should_stop.load(Ordering::Relaxed) {
            return board.score(color);
        }
        alpha = cmp::max(alpha, score);
        for mv in rest {
            if self.should_stop.load(Ordering::Relaxed) {
                break;
            }
            let next_board = board.flip(mv, color);
            let score = {
                let tmp_score =
                    -self.nega_scout(next_board, opposite, depth - 1, -alpha - 1, -alpha);
                self.check_time_limit();
                if self.should_stop.load(Ordering::Relaxed) {
                    break;
                }
                if alpha < tmp_score && tmp_score < beta {
                    -self.nega_scout(next_board, opposite, depth - 1, -beta, -tmp_score)
                } else {
                    tmp_score
                }
            };

            self.check_time_limit();
            alpha = cmp::max(alpha, score);

            // beta cut-off
            if alpha >= beta {
                break;
            }
        }
        alpha
    }

    #[inline]
    fn check_time_limit(&self) {
        if self.now.elapsed() > self.time_limit {
            println!("Timeout! Aborting.");
            self.should_stop.store(true, Ordering::Relaxed);
        }
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
