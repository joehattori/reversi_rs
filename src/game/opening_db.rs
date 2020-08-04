use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::{self, File};
use std::sync::RwLock;
use tar::Archive;

use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;

lazy_static! {
    pub static ref DARK_MOVES: RwLock<HashMap<Board, Square>> = RwLock::new(HashMap::new());
    pub static ref LIGHT_MOVES: RwLock<HashMap<Board, Square>> = RwLock::new(HashMap::new());
}

pub fn load_from_file() {
    let tar_gz = File::open("bin/opening_book.tar.gz").unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut ar = Archive::new(tar);
    ar.unpack(".").unwrap();
    println!("Unpacked tar");

    let mut dark_moves_count: HashMap<Board, HashMap<u8, i32>> = HashMap::new();
    let mut light_moves_count: HashMap<Board, HashMap<u8, i32>> = HashMap::new();

    let contents = fs::read_to_string("bin/opening_book.gam").unwrap();
    for line in contents.lines() {
        let mut board = Board::initial();
        let mut line = line.to_string();
        let winner = line.pop().unwrap();
        let target_count = {
            if winner == '+' {
                &mut dark_moves_count
            } else if winner == '-' {
                &mut light_moves_count
            } else {
                panic!("invalid");
            }
        };
        for (i, c) in line.chars().enumerate() {
            let color = {
                if i % 2 == 0 {
                    Color::Dark
                } else {
                    Color::Light
                }
            };
            let square = c as u8 - 33;
            if (winner == '+' && i % 2 == 0) || (winner == '-' && i % 2 == 1) {
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
            }
            board = board.flip(square, color);
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
    fs::remove_file("bin/opening_book.gam").unwrap();
}
