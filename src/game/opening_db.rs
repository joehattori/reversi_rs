use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;

use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;

lazy_static! {
    pub static ref DARK_MOVES: RwLock<HashMap<Board, Square>> = RwLock::new(HashMap::new());
    pub static ref LIGHT_MOVES: RwLock<HashMap<Board, Square>> = RwLock::new(HashMap::new());
}

pub fn load_from_file() {
    let mut dark_moves_count: HashMap<Board, HashMap<u8, i32>> = HashMap::new();
    let mut light_moves_count: HashMap<Board, HashMap<u8, i32>> = HashMap::new();

    let contents = fs::read_to_string("data/logbook.gam").unwrap();
    for line in contents.lines() {
        let mut board = Board::initial();
        let indicator = line.chars().collect::<Vec<char>>()[line.len() - 6];
        let (winner, target_count) = {
            if indicator == '+' {
                (Color::Dark, &mut dark_moves_count)
            } else if indicator == '-' {
                (Color::Light, &mut light_moves_count)
            } else {
                panic!("Invalid indicator {}", indicator)
            }
        };
        for i in 0..40 {
            let s = line[i * 3..(i + 1) * 3].to_string();
            let square = Square::from_str(&s[1..3]).unwrap().to_uint();
            if indicator != s.chars().next().unwrap() {
                board = board.flip(square, winner.opposite());
                continue;
            }
            target_count
                .entry(board)
                .and_modify(|hm| {
                    let _ = *hm.entry(square).and_modify(|c| *c += 1).or_insert(1);
                })
                .or_insert_with(|| {
                    let mut hm = HashMap::new();
                    hm.insert(square, 1);
                    hm
                });
            board = board.flip(square, winner);
        }
    }

    dark_moves_count.iter().for_each(|(&b, hm)| {
        let square: u8 = *hm
            .iter()
            .fold((None, 0), |(cur_s, cur_c), (s, c)| {
                if cur_c < *c {
                    (Some(s), *c)
                } else {
                    (cur_s, cur_c)
                }
            })
            .0
            .unwrap();
        let square = Square::from_uint(square);
        for (board, s) in [
            (b, square),
            (b.rotate_90(), square.rotate_90()),
            (b.rotate_180(), square.rotate_180()),
            (b.rotate_270(), square.rotate_270()),
        ]
        .iter()
        {
            DARK_MOVES.write().unwrap().insert(*board, *s);
        }

        let b = b.mirror();
        let square = square.mirror();
        for (board, s) in [
            (b, square),
            (b.rotate_90(), square.rotate_90()),
            (b.rotate_180(), square.rotate_180()),
            (b.rotate_270(), square.rotate_270()),
        ]
        .iter()
        {
            DARK_MOVES.write().unwrap().insert(*board, *s);
        }
    });

    light_moves_count.iter().for_each(|(&b, hm)| {
        let square: u8 = *hm
            .iter()
            .fold((None, 0), |(cur_s, cur_c), (s, c)| {
                if cur_c < *c {
                    (Some(s), *c)
                } else {
                    (cur_s, cur_c)
                }
            })
            .0
            .unwrap();
        let square = Square::from_uint(square);
        for (board, s) in [
            (b, square),
            (b.rotate_90(), square.rotate_90()),
            (b.rotate_180(), square.rotate_180()),
            (b.rotate_270(), square.rotate_270()),
        ]
        .iter()
        {
            LIGHT_MOVES.write().unwrap().insert(*board, *s);
        }

        let b = b.mirror();
        let square = square.mirror();
        for (board, s) in [
            (b, square),
            (b.rotate_90(), square.rotate_90()),
            (b.rotate_180(), square.rotate_180()),
            (b.rotate_270(), square.rotate_270()),
        ]
        .iter()
        {
            LIGHT_MOVES.write().unwrap().insert(*board, *s);
        }
    });
}
