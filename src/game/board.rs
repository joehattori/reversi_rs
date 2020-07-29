use crate::game::base::Color;
use crate::game::square::Square;
use crate::game::util::clz;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Board {
    pub dark: u64,
    pub light: u64,
}

impl Board {
    // TODO: move this part to other file.
    const MOUNTAIN_WEIGHT: (i16, i16) = (40, 20);
    const PURE_MOUNTAIN_WEIGHT: (i16, i16) = (60, 30);
    const CORNER_FLIPPABLE_WEIGHT: (i16, i16) = (100, 50);
    const WING_WEIGHT: (i16, i16) = (-40, -20);
    const SUB_WING_WEIGHT: (i16, i16) = (-20, -10);
    const SOLID_DISK_WEIGHT: (i16, i16) = (60, 30);
    const FLIPPABLE_COUNT_WEIGHT: (i16, i16) = (15, 40);
    const SQUARE_VALUES: [i16; 64] = [
        30, -12, 0, -1, -1, 0, -12, 30, -12, -15, -3, -3, -3, -3, -15, -12, 0, -3, 0, -1, -1, 0,
        -3, 0, -1, -3, -1, -1, -1, -1, -3, -1, -1, -3, -1, -1, -1, -1, -3, -1, 0, -3, 0, -1, -1, 0,
        -3, 0, -12, -15, -3, -3, -3, -3, -15, -12, 30, -12, 0, -1, -1, 0, -12, 30,
    ];

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

    #[inline]
    pub fn score(&self, color: Color) -> i16 {
        self.raw_score(color)
            + self.flippable_score(color)
            + self.flippable_count_score(color)
            + self.mountain_score(color)
            + self.corner_flippable_score(color)
            + self.wing_score(color)
            + self.solid_disks_score(color)
            + self.sub_wing_score(color)
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

    pub fn rotate_180(&self) -> Self {
        self.rotate_90().rotate_90()
    }

    pub fn rotate_270(&self) -> Self {
        self.rotate_90().rotate_90().rotate_90()
    }

    #[inline]
    fn mountain_score(&self, color: Color) -> i16 {
        let mut score = 0;

        if self.has_shape(color, 0x7e00000000000000) {
            score += if self.has_shape(color, 0x7e3c000000000000) {
                self.get_weight(Self::PURE_MOUNTAIN_WEIGHT)
            } else {
                self.get_weight(Self::MOUNTAIN_WEIGHT)
            }
        }
        if self.has_shape(color, 0x1010101010100) {
            score += if self.has_shape(color, 0x1030303030100) {
                self.get_weight(Self::PURE_MOUNTAIN_WEIGHT)
            } else {
                self.get_weight(Self::MOUNTAIN_WEIGHT)
            }
        }
        if self.has_shape(color, 0x7e) {
            score += if self.has_shape(color, 0x3c7e) {
                self.get_weight(Self::PURE_MOUNTAIN_WEIGHT)
            } else {
                self.get_weight(Self::MOUNTAIN_WEIGHT)
            }
        }
        if self.has_shape(color, 0x80808080808000) {
            score += if self.has_shape(color, 0x80c0c0c0c08000) {
                self.get_weight(Self::PURE_MOUNTAIN_WEIGHT)
            } else {
                self.get_weight(Self::MOUNTAIN_WEIGHT)
            }
        }
        score
    }

    #[inline]
    fn wing_score(&self, color: Color) -> i16 {
        let mut score = 0;
        if self.has_shape(color, 0x7c00000000000000) && !self.has_shape(color, 0x7e00000000000000) {
            score += self.get_weight(Self::WING_WEIGHT);
        }
        if self.has_shape(color, 0x1010101010000) && !self.has_shape(color, 0x1010101010100) {
            score += self.get_weight(Self::WING_WEIGHT);
        }
        if self.has_shape(color, 0x3e) && !self.has_shape(color, 0x7e) {
            score += self.get_weight(Self::WING_WEIGHT);
        }
        if self.has_shape(color, 0x808080808000) && !self.has_shape(color, 0x80808080808000) {
            score += self.get_weight(Self::WING_WEIGHT);
        }
        score
    }

    #[inline]
    fn sub_wing_score(&self, color: Color) -> i16 {
        let mut score = 0;
        if self.has_shape(color, 0x6) && !self.has_shape(color, 0xf) {
            score += self.get_weight(Self::SUB_WING_WEIGHT);
        }
        if self.has_shape(color, 0x808000) && !self.has_shape(color, 0x80808080) {
            score += self.get_weight(Self::SUB_WING_WEIGHT);
        }
        if self.has_shape(color, 0x6000000000000000) && !self.has_shape(color, 0xf000000000000000) {
            score += self.get_weight(Self::SUB_WING_WEIGHT);
        }
        if self.has_shape(color, 0x1010000000000) && !self.has_shape(color, 0x101010100000000) {
            score += self.get_weight(Self::SUB_WING_WEIGHT);
        }
        score
    }

    #[inline]
    fn corner_flippable_score(&self, color: Color) -> i16 {
        let mut score = 0;
        let flippables = self.flippable_squares(color);
        if flippables & 1 << 0 != 0 {
            score += self.get_weight(Self::CORNER_FLIPPABLE_WEIGHT);
        }
        if flippables & 1 << 7 != 0 {
            score += self.get_weight(Self::CORNER_FLIPPABLE_WEIGHT);
        }
        if flippables & 1 << 56 != 0 {
            score += self.get_weight(Self::CORNER_FLIPPABLE_WEIGHT);
        }
        if flippables & 1 << 63 != 0 {
            score += self.get_weight(Self::CORNER_FLIPPABLE_WEIGHT);
        }
        score
    }

    #[inline]
    fn flippable_score(&self, color: Color) -> i16 {
        let player_flippable = self.flippable_squares(color);
        let opponent_flippable = self.flippable_squares(color.opposite());
        (0..64)
            .filter(|i| player_flippable & 1_u64 << i != 0)
            .fold(0, |ret, i| ret + Self::SQUARE_VALUES[i])
            - (0..64)
                .filter(|i| opponent_flippable & 1_u64 << i != 0)
                .fold(0, |ret, i| ret + Self::SQUARE_VALUES[i])
    }

    #[inline]
    fn flippable_count_score(&self, color: Color) -> i16 {
        let (target, opponent) = self.target_boards(color);
        (target.count_ones() as i16 - opponent.count_ones() as i16)
            * self.get_weight(Self::FLIPPABLE_COUNT_WEIGHT)
    }

    #[inline]
    fn raw_score(&self, color: Color) -> i16 {
        let (target, opponent) = self.target_boards(color);
        (0..64)
            .filter(|i| target & 1_u64 << i != 0)
            .fold(0, |ret, i| ret + Self::SQUARE_VALUES[i])
            - (0..64)
                .filter(|i| opponent & 1_u64 << i != 0)
                .fold(0, |ret, i| ret + Self::SQUARE_VALUES[i])
    }

    fn solid_disks_score(&self, color: Color) -> i16 {
        (self.solid_disks_count(color) - self.solid_disks_count(color.opposite())) as i16
            * self.get_weight(Self::SOLID_DISK_WEIGHT)
    }

    // TODO: fix double count
    fn solid_disks_count(&self, color: Color) -> i8 {
        let board = self.disks_of_color(color);
        let corners: [u64; 4] = [0, 7, 56, 63];
        let corners_dirs: [[i8; 2]; 4] = [[8, 1], [8, -1], [-8, 1], [-8, -1]];
        corners
            .iter()
            .zip(corners_dirs.iter())
            .fold(0, |ret, (corner, dirs)| {
                ret + if board & 1_u64 << corner != 0 {
                    dirs.iter().fold(1, |acum, dir| {
                        acum + self.solid_disks_line(color, *corner as i8, *dir, 6)
                    })
                } else {
                    0
                }
            })
    }

    #[inline]
    fn target_boards(&self, color: Color) -> (u64, u64) {
        match color {
            Color::Dark => (self.dark, self.light),
            Color::Light => (self.light, self.dark),
        }
    }

    fn solid_disks_line(&self, color: Color, square: i8, diff: i8, dep: u8) -> i8 {
        let new_square = square + diff;
        if dep == 0 {
            0
        } else if self.disks_of_color(color) & 1 << new_square != 0 {
            self.solid_disks_line(color, new_square, diff, dep - 1) + 1
        } else {
            0
        }
    }

    #[inline]
    fn has_shape(&self, color: Color, shape: u64) -> bool {
        self.disks_of_color(color) & shape == shape
    }

    #[inline]
    fn disks_of_color(&self, color: Color) -> u64 {
        match color {
            Color::Dark => self.dark,
            Color::Light => self.light,
        }
    }

    #[inline]
    fn get_weight(&self, weight: (i16, i16)) -> i16 {
        if self.empty_squares_count() > 30 {
            weight.0
        } else {
            weight.1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winnable_color() {
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

    #[test]
    fn mountain_score() {
        let board = Board {
            dark: 0x7e0181818181817e,
            light: 0,
        };
        board.print();
        assert_eq!(
            board.mountain_score(Color::Dark),
            board.get_weight(Board::MOUNTAIN_WEIGHT) * 3
        );
    }

    #[test]
    fn solid_disks_count() {
        let board = Board::initial();
        assert_eq!(board.solid_disks_count(Color::Light), 0);

        let board = Board {
            dark: 0x0000783c465c3c7e,
            light: 0x008080c0b8a0c080,
        };
        assert_eq!(board.solid_disks_count(Color::Light), 7);

        let board = Board {
            dark: 0x0000783c465c3c7e,
            light: 0x008080c0b8a04080,
        };
        assert_eq!(board.solid_disks_count(Color::Light), 1);

        // TODO: fix this case
        //let board = board.flip(Square::from_str("A1").unwrap(), Color::Light);
        //board.print();
        //assert_eq!(board.solid_disks_count(Color::Light), 13);
    }
}
