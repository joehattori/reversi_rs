use std::cmp;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::opening_db::{DARK_MOVES, LIGHT_MOVES};
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct NegaScout {
    pub should_stop: AtomicBool,
    pub now: Instant,
    pub time_limit: Duration,
    pub emergency_ret: Option<u8>,
}

impl Strategy for NegaScout {
    fn next_move(&self, board: Board, color: Color) -> Option<Square> {
        let moves = match color {
            Color::Dark => DARK_MOVES.read().unwrap(),
            Color::Light => LIGHT_MOVES.read().unwrap(),
        };
        if let Some(m) = moves.get(&board) {
            return Some(*m);
        }
        let flippables = board.flippable_squares(color);
        if flippables == 0 {
            return None;
        }
        let count = flippables.count_ones();

        let depth = if count < 4 {
            9
        } else if count < 8 {
            8
        } else {
            6
        };

        let mut ret = self.emergency_ret;
        let mut cur_max = -5000;

        for cur_square in (0..64).filter(|&x| flippables & 1 << x != 0) {
            if self.should_stop.load(Ordering::Relaxed) {
                break;
            }
            self.check_time_limit();
            let score = self.nega_scout(board, cur_square, color, depth, -5000, 5000);
            if cur_max < score {
                cur_max = score;
                ret = Some(cur_square);
            } else {
                ret = ret.or(Some(cur_square));
            }
        }
        ret.map(|x| Square::from_uint(x))
    }
}

impl NegaScout {
    pub fn new(time_limit_millisec: u64, emergency_ret: Option<u8>) -> Self {
        Self {
            should_stop: AtomicBool::new(false),
            now: Instant::now(),
            time_limit: Duration::from_millis(time_limit_millisec),
            emergency_ret,
        }
    }

    pub fn new_from_duration(duration: Duration, emergency_ret: Option<u8>) -> Self {
        Self {
            should_stop: AtomicBool::new(false),
            now: Instant::now(),
            time_limit: duration,
            emergency_ret,
        }
    }

    #[inline]
    pub fn emergency_move(board: Board, color: Color) -> Option<u8> {
        let flippables = board.flippable_squares(color);
        //TODO: improve
        (0..64)
            .filter(|&s| flippables & 1 << s != 0)
            .max_by_key(|&x| board.score(x, color))
    }

    fn nega_scout(
        &self,
        board: Board,
        next_move: u8,
        color: Color,
        depth: i8,
        mut alpha: i16,
        mut beta: i16,
    ) -> i16 {
        let next_board = board.flip(next_move, color);
        let opposite = color.opposite();
        let flippables = next_board.flippable_squares(opposite);
        if depth == 0 || flippables == 0 || self.should_stop.load(Ordering::Relaxed) {
            return board.score(next_move, color);
        }

        self.check_time_limit();
        if self.should_stop.load(Ordering::Relaxed) {
            return board.score(next_move, color);
        }

        let moves = Self::order_moves(next_board, opposite, flippables);

        for (i, &mv) in moves.iter().enumerate() {
            if self.should_stop.load(Ordering::Relaxed) {
                return alpha;
            }

            let score = -self.nega_scout(next_board, mv, opposite, depth - 1, -beta, -alpha);

            self.check_time_limit();
            if self.should_stop.load(Ordering::Relaxed) {
                return alpha;
            }

            if alpha < score && score < beta && i > 1 && depth > 1 {
                alpha = -self.nega_scout(next_board, mv, opposite, depth - 1, -beta, -score);
            }

            alpha = cmp::max(alpha, score);

            // beta cut-off
            if alpha >= beta {
                return alpha;
            }
            beta = alpha + 1;
            self.check_time_limit();
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
    fn order_moves(board: Board, color: Color, flippables: u64) -> Vec<u8> {
        let mut flippables: Vec<u8> = (0..64).filter(|&s| flippables & 1 << s != 0).collect();
        // using nega_scout is maybe too slow?
        flippables.sort_by(|x, y| {
            let x = board.score(*x, color);
            let y = board.score(*y, color);
            x.partial_cmp(&y).unwrap()
        });
        flippables
    }
}
