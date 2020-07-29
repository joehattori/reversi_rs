use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use super::NegaScout;
use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct Exhausive {
    pub should_stop: AtomicBool,
    pub time_limit: Duration,
    pub now: Instant,
}

impl Strategy for Exhausive {
    fn next_move(&self, board: Board, color: Color) -> Option<Square> {
        let flippables = board.flippable_squares(color);
        if flippables == 0 {
            return None;
        }
        let mut ret = None;
        for square in order_moves(board, color, flippables) {
            // NEXT: hash (able to store answers along searching)
            let next_board = board.flip(square, color);
            match self.winnable_color(next_board, color.opposite(), false) {
                Some(c) => {
                    if c == color {
                        return Some(Square::from_uint(square));
                    }
                }
                None => ret = ret.or(Some(Square::from_uint(square))),
            }

            self.check_time_limit();
            if self.should_stop.load(Ordering::Relaxed) {
                return self.switch_to_nega_scout(board, color);
            }
        }
        if ret.is_none() {
            println!("LOSE color: {:?}", color);
            board.print();
        }
        ret.or(Some(Square::from_uint(flippables.trailing_zeros() as u8)))
    }
}

impl Exhausive {
    pub fn new(time_limit_millisec: u64) -> Self {
        Self {
            should_stop: AtomicBool::new(false),
            now: Instant::now(),
            time_limit: Duration::from_millis(time_limit_millisec / 2),
        }
    }

    fn winnable_color(&self, board: Board, hand: Color, passed: bool) -> Option<Color> {
        if board.is_end() {
            return board.winner();
        } else if board.is_last_move() {
            return self.winnable_color_last(board, hand, passed);
        }

        self.check_time_limit();
        if self.should_stop.load(Ordering::Relaxed) {
            return None;
        }

        let mut flippables = board.flippable_squares(hand);
        let opposite = hand.opposite();
        if flippables == 0 {
            return if passed {
                board.winner()
            } else {
                self.winnable_color(board, opposite, true)
            };
        }

        let mut ret = Some(hand.opposite());
        let mut pos = 0;
        while flippables > 0 {
            let zeros = flippables.trailing_zeros() as u8;
            if zeros < 63 {
                flippables >>= zeros + 1;
                pos += zeros + 1;
            } else {
                flippables = 0;
                pos = 64;
            };
            let next_board = board.flip(pos - 1, hand);
            match self.winnable_color(next_board, opposite, false) {
                Some(c) => {
                    if c == hand {
                        return Some(hand);
                    }
                }
                None => ret = None,
            }

            self.check_time_limit();
            if self.should_stop.load(Ordering::Relaxed) {
                return None;
            }
        }
        ret
    }

    fn winnable_color_last(&self, board: Board, hand: Color, passed: bool) -> Option<Color> {
        let flippables = board.flippable_squares(hand);
        if flippables == 0 {
            if passed {
                board.winner()
            } else {
                let opposite = hand.opposite();
                let flippables = board.flippable_squares(opposite);
                if flippables == 0 {
                    board.winner()
                } else {
                    let pos = flippables.trailing_zeros() as u8;
                    board.flip(pos, opposite).winner()
                }
            }
        } else {
            let pos = flippables.trailing_zeros() as u8;
            board.flip(pos, hand).winner()
        }
    }

    fn check_time_limit(&self) {
        if self.now.elapsed() > self.time_limit {
            println!("Timeout! Switching...");
            self.should_stop.store(true, Ordering::Relaxed);
        }
    }

    fn switch_to_nega_scout(&self, board: Board, color: Color) -> Option<Square> {
        NegaScout::new_from_duration(self.time_limit / 2).next_move(board, color)
    }
}

fn order_moves(board: Board, color: Color, flippables: u64) -> Vec<u8> {
    let mut ret = (0..64)
        .filter(|&x| flippables & 1 << x != 0)
        .collect::<Vec<u8>>();
    ret.sort_by(|a, b| {
        let a_score = board.flip(*a, color).score(color);
        let b_score = board.flip(*b, color).score(color);
        b_score.partial_cmp(&a_score).unwrap()
    });
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_move() {
        let boards = [
            Board {
                dark: 0x6000100810120500,
                light: 0x8efceff76f6d3a3f,
            },
            Board {
                dark: 0xfc2eeeb28a8c2e3e,
                light: 0x0311114d75735100,
            },
        ];
        let next_moves = ["E8", "H1"];
        let e = Exhausive::new(100000);
        for (b, s) in boards.iter().zip(next_moves.iter()) {
            assert_eq!(
                e.next_move(b.clone(), Color::Dark).unwrap().to_string(),
                s.to_string()
            );
        }
    }
}
