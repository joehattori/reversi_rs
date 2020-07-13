use crate::game::common::Color;
use crate::game::square::Square;
use crate::game::util::clz;

#[derive(Copy, Clone)]
pub struct Board {
    pub dark: u64,
    pub light: u64,
}

impl Board {
    pub fn initial() -> Board {
        Board {
            dark: 1u64 << Square::from_str("D5").unwrap().to_uint()
                | 1u64 << Square::from_str("E4").unwrap().to_uint(),
            light: 1u64 << Square::from_str("D4").unwrap().to_uint()
                | 1u64 << Square::from_str("E5").unwrap().to_uint(),
        }
    }

    pub fn print(&self) {
        println!(" |A B C D E F G H");
        println!("-+---------------");
        for i in 0..64 {
            if i % 8 == 0 {
                if i > 0 {
                    print!("\n");
                }
                print!("{}|", i / 8 + 1);
            }

            if self.dark & 1 << i != 0 {
                print!("x ");
            } else if self.light & 1 << i != 0 {
                print!("o ");
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }

    pub fn flip(&self, pos: &Square, color: Color) -> Board {
        let flipped = self.flipped_squares(&pos, color);
        match color {
            Color::Dark => Board {
                dark: self.dark | 1u64 << pos.to_uint() | flipped,
                light: self.light & !flipped,
            },
            Color::Light => Board {
                dark: self.dark & !flipped,
                light: self.light | 1u64 << pos.to_uint() | flipped,
            },
        }
    }

    pub fn flippable_squares(&self, color: Color) -> u64 {
        let (target_board, other_board) = self.target_boards(color);

        let horizontal_watcher = other_board & 0x7e7e7e7e7e7e7e7e;
        let vertical_watcher = other_board & 0x00ffffffffffff00;
        let sides_watcher = other_board & 0x007e7e7e7e7e7e00;
        let blank_squares = !(target_board | other_board);

        // one can flip atmost 6 disks.
        // opening for loops for speed up

        // west
        let mut tmp = horizontal_watcher & (target_board << 1);
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        let legal_west = blank_squares & tmp << 1;

        // east
        let mut tmp = horizontal_watcher & target_board >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        let legal_east = blank_squares & tmp >> 1;

        // top
        let mut tmp = vertical_watcher & target_board << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        let legal_north = blank_squares & tmp << 8;

        // bottom
        let mut tmp = vertical_watcher & target_board >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        let legal_south = blank_squares & tmp >> 8;

        // north west
        let mut tmp = sides_watcher & target_board << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        let legal_north_west = blank_squares & tmp << 9;

        // north east
        let mut tmp = sides_watcher & target_board << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        let legal_north_east = blank_squares & tmp << 7;

        // south west
        let mut tmp = sides_watcher & target_board >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        let legal_south_west = blank_squares & tmp >> 7;

        // south east
        let mut tmp = sides_watcher & target_board >> 9;
        tmp |= sides_watcher & tmp >> 9;
        tmp |= sides_watcher & tmp >> 9;
        tmp |= sides_watcher & tmp >> 9;
        tmp |= sides_watcher & tmp >> 9;
        tmp |= sides_watcher & tmp >> 9;
        let legal_south_east = blank_squares & tmp >> 9;

        legal_west
            | legal_east
            | legal_north
            | legal_south
            | legal_north_west
            | legal_north_east
            | legal_south_west
            | legal_south_east
    }

    pub fn flipped_squares(&self, square: &Square, color: Color) -> u64 {
        let (target_board, mut other_board) = self.target_boards(color);
        let square_uint = square.to_uint();

        let mut ret = 0u64;

        let mut mask = 0x0080808080808080u64 >> (63 - square_uint);
        let mut outflank = 0x8000000000000000u64 >> clz(!other_board & mask) & target_board;
        let mut flipped = (-(outflank as i128) * 2) as u64 & mask;
        mask = 0x0101010101010100u64 << square_uint;
        outflank = mask & ((other_board | !mask) as u128 + 1) as u64 & target_board;
        flipped |= (outflank as i128 - ((outflank != 0) as i128)) as u64 & mask;
        ret |= flipped;

        other_board &= 0x7e7e7e7e7e7e7e7eu64;

        let mut mask = 0x7f00000000000000u64 >> (63 - square_uint);
        let mut outflank = 0x8000000000000000u64 >> clz(!other_board & mask) & target_board;
        let mut flipped = (-(outflank as i128) * 2) as u64 & mask;
        mask = 0x00000000000000feu64 << square_uint;
        outflank = mask & ((other_board | !mask) as u128 + 1) as u64 & target_board;
        flipped |= (outflank as i128 - ((outflank != 0) as i128)) as u64 & mask;
        ret |= flipped;

        let mut mask = 0x0102040810204000u64 >> (63 - square_uint);
        let mut outflank = 0x8000000000000000u64 >> clz(!other_board & mask) & target_board;
        let mut flipped = (-(outflank as i128) * 2) as u64 & mask;
        mask = 0x0002040810204080u64 << square_uint;
        outflank = mask & ((other_board | !mask) as u128 + 1) as u64 & target_board;
        flipped |= (outflank as i128 - ((outflank != 0) as i128)) as u64 & mask;
        ret |= flipped;

        let mut mask = 0x0040201008040201u64 >> (63 - square_uint);
        let mut outflank = 0x8000000000000000u64 >> clz(!other_board & mask) & target_board;
        let mut flipped = (-(outflank as i128) * 2) as u64 & mask;
        mask = 0x8040201008040200u64 << square_uint;
        outflank = mask & ((other_board | !mask) as u128 + 1) as u64 & target_board;
        flipped |= (outflank as i128 - ((outflank != 0) as i128)) as u64 & mask;
        ret |= flipped;

        ret
    }

    pub fn winnable_color(&self, hand: Color, passed: bool) -> Option<Color> {
        if self.is_end() {
            return Some(self.winner());
        }
        let mut flippables = self.flippable_squares(hand);
        let opposite = hand.opposite();
        if flippables == 0 {
            if passed {
                return Some(self.winner());
            }
            return self.winnable_color(opposite, true);
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
            match self
                .flip(&Square::from_uint(pos - 1), hand)
                .winnable_color(opposite, false)
            {
                Some(c) => {
                    if c == hand {
                        return Some(hand);
                    }
                }
                None => ret = None,
            }
        }
        ret
    }

    pub fn empty_squares_count(&self) -> u8 {
        (self.dark | self.light).count_zeros() as u8
    }

    fn target_boards(&self, color: Color) -> (u64, u64) {
        match color {
            Color::Dark => (self.dark, self.light),
            Color::Light => (self.light, self.dark),
        }
    }

    fn winner(&self) -> Color {
        if self.dark.count_ones() > self.light.count_ones() {
            Color::Dark
        } else {
            Color::Light
        }
    }

    fn is_end(&self) -> bool {
        (self.dark & self.light).count_ones() == 64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winnable_color_test() {
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
        let results = [Color::Dark, Color::Dark];
        boards
            .iter()
            .zip(results.iter())
            .for_each(|(b, r)| assert_eq!(b.winnable_color(Color::Dark, false).unwrap(), *r));
    }
}
