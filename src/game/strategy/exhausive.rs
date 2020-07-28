use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use super::NegaScout;
use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct Exhausive();

impl Strategy for Exhausive {
    fn next_move(&self, board: Board, color: Color, time: i32) -> Option<Square> {
        let now = Instant::now();
        let flippables = board.flippable_squares(color);
        if flippables == 0 {
            return None;
        }
        let mut ret = None;
        for square in order_moves(board, color, flippables) {
            if now.elapsed() > Duration::from_millis(time as u64 / 3) {
                return switch_to_nega_scout(board, color, time - now.elapsed().as_millis() as i32);
            }
            match board
                .flip(square, color)
                .winnable_color(color.opposite(), false)
            {
                Some(c) => {
                    if c == color {
                        return Some(Square::from_uint(square));
                    }
                }
                None => ret = ret.or(Some(Square::from_uint(square))),
            }
        }
        if ret.is_none() {
            println!("LOSE color: {:?}", color);
            board.print();
        }
        ret.or(Some(Square::from_uint(flippables.trailing_zeros() as u8)))
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

fn switch_to_nega_scout(board: Board, color: Color, time: i32) -> Option<Square> {
    NegaScout {
        should_stop: AtomicBool::new(false),
    }
    .next_move(board, color, time)
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
        let e = Exhausive();
        for (b, s) in boards.iter().zip(next_moves.iter()) {
            assert_eq!(
                e.next_move(b.clone(), Color::Dark, 0).unwrap().to_string(),
                s.to_string()
            );
        }
    }
}
