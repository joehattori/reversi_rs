use crate::game::base::Color;
use crate::game::square::Square;
use crate::game::util::clz;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Board {
    pub dark: u64,
    pub light: u64,
}

impl Board {
    pub fn initial() -> Self {
        Self {
            dark: 1u64 << Square::from_str("D5").unwrap().to_uint()
                | 1u64 << Square::from_str("E4").unwrap().to_uint(),
            light: 1u64 << Square::from_str("D4").unwrap().to_uint()
                | 1u64 << Square::from_str("E5").unwrap().to_uint(),
        }
    }

    #[allow(dead_code)]
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

    #[inline]
    pub fn flip(&self, square: u8, color: Color) -> Self {
        let flipped = self.flipped_squares(square, color);
        match color {
            Color::Dark => Self {
                dark: self.dark | 1u64 << square | flipped,
                light: self.light & !flipped,
            },
            Color::Light => Self {
                dark: self.dark & !flipped,
                light: self.light | 1u64 << square | flipped,
            },
        }
    }

    #[inline]
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

    #[inline]
    pub fn flipped_squares(&self, square_uint: u8, color: Color) -> u64 {
        let (target_board, mut other_board) = self.target_boards(color);

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

    #[inline]
    pub fn empty_squares_count(&self) -> u8 {
        (self.dark | self.light).count_zeros() as u8
    }

    pub fn rotate_90(&self) -> Self {
        let tmp = self.light;
        let mut light = 0x00000000f0f0f0f0u64 & (tmp << 4);
        light |= 0xf0f0f0f00f0f0f0fu64 & (tmp << 32);
        light |= 0xf0f0f0f00f0f0f0fu64 & (tmp >> 32);
        light |= 0x0f0f0f0f00000000u64 & (tmp >> 4);

        let tmp = light;
        light = 0x0000cccc0000cccc & (tmp << 2);
        light |= 0xcccc0000cccc0000 & (tmp << 16);
        light |= 0x0000333300003333 & (tmp >> 16);
        light |= 0x3333000033330000 & (tmp >> 2);

        let tmp = light;
        light = 0x00aa00aa00aa00aa & (tmp << 1);
        light |= 0xaa00aa00aa00aa00 & (tmp << 8);
        light |= 0x0055005500550055 & (tmp >> 8);
        light |= 0x5500550055005500 & (tmp >> 1);

        let tmp = self.dark;
        let mut dark = 0x00000000f0f0f0f0u64 & (tmp << 4);
        dark |= 0xf0f0f0f00f0f0f0fu64 & (tmp << 32);
        dark |= 0xf0f0f0f00f0f0f0fu64 & (tmp >> 32);
        dark |= 0x0f0f0f0f00000000u64 & (tmp >> 4);

        let tmp = dark;
        dark = 0x0000cccc0000cccc & (tmp << 2);
        dark |= 0xcccc0000cccc0000 & (tmp << 16);
        dark |= 0x0000333300003333 & (tmp >> 16);
        dark |= 0x3333000033330000 & (tmp >> 2);

        let tmp = dark;
        dark = 0x00aa00aa00aa00aa & (tmp << 1);
        dark |= 0xaa00aa00aa00aa00 & (tmp << 8);
        dark |= 0x0055005500550055 & (tmp >> 8);
        dark |= 0x5500550055005500 & (tmp >> 1);

        Self { light, dark }
    }

    pub fn rotate_180(&self) -> Self {
        self.rotate_90().rotate_90()
    }

    pub fn rotate_270(&self) -> Self {
        self.rotate_90().rotate_90().rotate_90()
    }

    pub fn mirror(&self) -> Self {
        let mut tmp = self.dark;
        tmp = ((tmp >> 8) & 0x00ff00ff00ff00ff) | ((tmp & 0x00ff00ff00ff00ff) << 8);
        tmp = ((tmp >> 16) & 0x0000ffff0000ffff) | ((tmp & 0x0000ffff0000ffff) << 16);
        tmp = (tmp >> 32) | (tmp << 32);
        let dark = tmp;

        let mut tmp = self.light;
        tmp = ((tmp >> 8) & 0x00ff00ff00ff00ff) | ((tmp & 0x00ff00ff00ff00ff) << 8);
        tmp = ((tmp >> 16) & 0x0000ffff0000ffff) | ((tmp & 0x0000ffff0000ffff) << 16);
        tmp = (tmp >> 32) | (tmp << 32);
        let light = tmp;

        Self { dark, light }
    }

    #[inline]
    pub fn winner(&self) -> Option<Color> {
        if self.dark.count_ones() > self.light.count_ones() {
            Some(Color::Dark)
        } else if self.dark.count_ones() < self.light.count_ones() {
            Some(Color::Light)
        } else {
            None
        }
    }

    #[inline]
    pub fn is_last_move(&self) -> bool {
        (self.dark | self.light).count_zeros() == 1
    }

    #[inline]
    pub fn is_end(&self) -> bool {
        (self.dark | self.light).count_zeros() == 0
    }

    #[inline]
    pub fn target_boards(&self, color: Color) -> (u64, u64) {
        match color {
            Color::Dark => (self.dark, self.light),
            Color::Light => (self.light, self.dark),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror() {
        let board = Board {
            dark: 0x7844444870504844,
            light: 0x0,
        };
        let expected = Board {
            dark: 0x4448507048444478,
            light: 0x0,
        };
        assert_eq!(board.mirror(), expected);
    }
}
