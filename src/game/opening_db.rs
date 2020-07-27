use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use crate::game::base::Color;
use crate::game::board::Board;
use crate::game::square::Square;

lazy_static! {
    pub static ref DARK_MOVES: Mutex<HashMap<Board, Square>> = Mutex::new(HashMap::new());
    pub static ref LIGHT_MOVES: Mutex<HashMap<Board, Square>> = Mutex::new(HashMap::new());
}

pub fn load_from_file() {
    let mut dark_moves_count: HashMap<Board, HashMap<Square, i32>> = HashMap::new();
    let mut light_moves_count: HashMap<Board, HashMap<Square, i32>> = HashMap::new();

    let contents = fs::read_to_string("logbook.gam").unwrap();
    for line in contents.lines() {
        let mut board = Board::initial();
        let indicator = line.chars().collect::<Vec<char>>()[line.len() - 6];
        let winner = {
            if indicator == '+' {
                Color::Dark
            } else if indicator == '-' {
                Color::Light
            } else {
                panic!("Invalid indicator {}", indicator)
            }
        };
        for i in 0..20 {
            let s: String = line[i * 3..(i + 1) * 3].to_string();
            if indicator != s.chars().collect::<Vec<char>>()[0] {
                continue;
            }
            let square = Square::from_str(&s[1..3]).unwrap();
            let target_count = match winner {
                Color::Dark => &mut dark_moves_count,
                Color::Light => &mut light_moves_count,
            };
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

    dark_moves_count.iter().for_each(|(k, hm)| {
        let square: Square = *hm
            .iter()
            .fold((None, 0), |(mut cur_s, mut cur_c), (s, c)| {
                if cur_c < *c {
                    cur_c = *c;
                    cur_s = Some(s);
                }
                (cur_s, cur_c)
            })
            .0
            .unwrap();
        DARK_MOVES.lock().unwrap().insert(*k, square);
    });

    light_moves_count.iter().for_each(|(k, hm)| {
        let square: Square = *hm
            .iter()
            .fold((None, 0), |(mut cur_s, mut cur_c), (s, c)| {
                if cur_c < *c {
                    cur_c = *c;
                    cur_s = Some(s);
                }
                (cur_s, cur_c)
            })
            .0
            .unwrap();
        LIGHT_MOVES.lock().unwrap().insert(*k, square);
    });
    println!("len: {}", DARK_MOVES.lock().unwrap().len());
    println!("len: {}", LIGHT_MOVES.lock().unwrap().len());
}
