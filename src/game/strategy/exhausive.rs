use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;
use crate::game::strategy::Strategy;

pub struct Exhausive();

impl Strategy for Exhausive {
    fn next_move(&self, board: Board, color: Color, _: i32) -> Option<Square> {
        let flippables = board.flippable_squares(color);
        if flippables == 0 {
            return None;
        }
        let mut ret = None;
        for i in (0..64).filter(|&x| flippables & 1 << x != 0) {
            let square = Square::from_uint(i);
            match board
                .flip(square, color)
                .winnable_color(color.opposite(), false)
            {
                Some(c) => {
                    if c == color {
                        return Some(square);
                    }
                }
                None => ret = ret.or(Some(square)),
            }
        }
        if ret.is_none() {
            println!("LOSE color: {:?}", color);
            board.print();
        }
        ret.or(Some(Square::from_uint(flippables.trailing_zeros() as u8)))
    }
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
