use super::board::Board;
use crate::game::base::Color;

impl Board {
    // TODO: polish
    const MOUNTAIN_WEIGHT: [i16; 3] = [20, 20, 10];
    const PURE_MOUNTAIN_WEIGHT: [i16; 3] = [50, 30, 20];
    const WING_WEIGHT: [i16; 3] = [-10, -10, -1];
    const SUB_WING_WEIGHT: [i16; 3] = [-5, -5, -1];
    const CORNER_FLIPPABLE_WEIGHT: [i16; 3] = [-80, -80, -80];
    const SOLID_DISK_WEIGHT: [i16; 3] = [8, 8, 8];
    const FLIPPABLE_COUNT_WEIGHT: [i16; 3] = [-3, -1, -1];
    const OPENNESS_WEIGHT: [i16; 3] = [-3, -2, -2];

    //100, -40,  1, -1, -1,  1, -40, 100,
    //-40, -80, -3, -3, -3, -3, -80, -40,
    //1,    -3,  1, -1, -1,  1,  -3,   1,
    //-1,   -3, -1,  0,  0, -1,  -3,  -1,
    //-1,   -3, -1,  0,  0, -1,  -3,  -1,
    //1,    -3,  1, -1, -1,  1,  -3,   1,
    //-40, -80, -3, -3, -3, -3, -80, -40,
    //100, -40,  1, -1, -1,  1, -40, 100,
    const RAW_VALUES: [i16; 64] = [
        100, -40, 1, -1, -1, 1, -40, 100, -40, -80, -3, -3, -3, -3, -80, -40, 1, -3, 1, -1, -1, 1,
        -3, 1, -1, -3, -1, 0, 0, -1, -3, -1, -1, -3, -1, 0, 0, -1, -3, -1, 1, -3, 1, -1, -1, 1, -3,
        1, -40, -80, -3, -3, -3, -3, -80, -40, 100, -40, 1, -1, -1, 1, -40, 100,
    ];

    #[inline]
    // NEXT revise and check raw score
    pub fn score(&self, next_move: u8, color: Color) -> i16 {
        let opposite = color.opposite();
        let next_board = self.flip(next_move, color);
        next_board.raw_score(color)
            + next_board.flippable_count_score(opposite)
            + next_board.corner_flippable_score(opposite)
            + next_board.mountain_score(color)
            //+ next_board.wing_score(color)
            //+ next_board.sub_wing_score(color)
            + next_board.solid_disks_score(color)
            + self.openness_score(next_move, color)
            + next_board.empty_score(color)
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
        let mut count = 0;
        if self.has_shape(color, 0x7c00000000000000) && !self.has_shape(color, 0x7e00000000000000) {
            count += 1;
        }
        if self.has_shape(color, 0x1010101010000) && !self.has_shape(color, 0x1010101010100) {
            count += 1;
        }
        if self.has_shape(color, 0x3e) && !self.has_shape(color, 0x7e) {
            count += 1;
        }
        if self.has_shape(color, 0x808080808000) && !self.has_shape(color, 0x80808080808000) {
            count += 1;
        }
        count * self.get_weight(Self::WING_WEIGHT)
    }

    #[inline]
    fn sub_wing_score(&self, color: Color) -> i16 {
        let mut count = 0;
        if self.has_shape(color, 0x6) && !self.has_shape(color, 0xf) {
            count += 1;
        }
        if self.has_shape(color, 0x808000) && !self.has_shape(color, 0x80808080) {
            count += 1;
        }
        if self.has_shape(color, 0x6000000000000000) && !self.has_shape(color, 0xf000000000000000) {
            count += 1;
        }
        if self.has_shape(color, 0x1010000000000) && !self.has_shape(color, 0x101010100000000) {
            count += 1;
        }
        count * self.get_weight(Self::SUB_WING_WEIGHT)
    }

    #[inline]
    fn corner_flippable_score(&self, color: Color) -> i16 {
        let mut count = 0;
        let flippables = self.flippable_squares(color);
        if flippables & 1 << 0 != 0 {
            count += 1;
        }
        if flippables & 1 << 7 != 0 {
            count += 1;
        }
        if flippables & 1 << 56 != 0 {
            count += 1;
        }
        if flippables & 1 << 63 != 0 {
            count += 1;
        }
        count * self.get_weight(Self::CORNER_FLIPPABLE_WEIGHT)
    }

    #[inline]
    fn flippable_count_score(&self, color: Color) -> i16 {
        //let player_flippable = self.flippable_squares(color);
        //let opponent_flippable = self.flippable_squares(color.opposite());
        //(player_flippable.count_ones() as i16 - 2 * opponent_flippable.count_ones() as i16)
        //* self.get_weight(Self::FLIPPABLE_COUNT_WEIGHT)
        self.flippable_squares(color).count_ones() as i16
            * self.get_weight(Self::FLIPPABLE_COUNT_WEIGHT)
    }

    #[inline]
    fn openness_score(&self, mv: u8, color: Color) -> i16 {
        let flipped = self.flipped_squares(mv, color);
        let openness = (0..64)
            .filter(|x| flipped & 1_u64 << x != 0)
            .fold(0_u64, |ret, s| ret + self.openness_of_square(s)) as i16;
        openness * self.get_weight(Self::OPENNESS_WEIGHT)
    }

    #[inline]
    fn openness_of_square(&self, square: u8) -> u64 {
        let square = 1_u64 << square as u64;
        let blank = !(self.dark | self.light);
        let mut bb = (square << 1 & (blank & 0xfefefefefefefefe))
            | (square >> 1 & (blank & 0x7f7f7f7f7f7f7f7f))
            | (square << 8 & (blank & 0xffffffffffffffff))
            | (square >> 8 & (blank & 0xffffffffffffffff))
            | (square << 7 & (blank & 0x7f7f7f7f7f7f7f7f))
            | (square >> 7 & (blank & 0xfefefefefefefefe))
            | (square << 9 & (blank & 0xfefefefefefefefe))
            | (square >> 9 & (blank & 0x7f7f7f7f7f7f7f7f));
        bb = (bb & 0x5555555555555555) + (bb >> 1 & 0x5555555555555555);
        bb = (bb & 0x3333333333333333) + (bb >> 2 & 0x3333333333333333);
        bb = (bb & 0x0f0f0f0f0f0f0f0f) + (bb >> 4 & 0x0f0f0f0f0f0f0f0f);
        bb = (bb & 0x00ff00ff00ff00ff) + (bb >> 8 & 0x00ff00ff00ff00ff);
        bb = (bb & 0x0000ffff0000ffff) + (bb >> 16 & 0x0000ffff0000ffff);
        (bb & 0x00000000ffffffff) + (bb >> 32 & 0x00000000ffffffff)
    }

    #[inline]
    fn raw_score(&self, color: Color) -> i16 {
        let (target, opponent) = self.target_boards(color);
        (0..64)
            .filter(|i| target & 1_u64 << i != 0)
            .fold(0, |ret, i| ret + Self::RAW_VALUES[i])
            - (0..64)
                .filter(|i| opponent & 1_u64 << i != 0)
                .fold(0, |ret, i| ret + Self::RAW_VALUES[i])
    }

    fn solid_disks_score(&self, color: Color) -> i16 {
        (self.solid_disks_count(color) - self.solid_disks_count(color.opposite())) as i16
            * self.get_weight(Self::SOLID_DISK_WEIGHT)
    }

    // TODO: fix double count
    fn solid_disks_count(&self, color: Color) -> i8 {
        let (player_board, opponent_board) = self.target_boards(color);
        let corners: [u64; 4] = [0, 7, 56, 63];
        let corners_dirs: [[i8; 2]; 4] = [[8, 1], [8, -1], [-8, 1], [-8, -1]];
        corners
            .iter()
            .zip(corners_dirs.iter())
            .fold(0, |ret, (corner, dirs)| {
                ret + if player_board & 1_u64 << corner != 0 {
                    dirs.iter().fold(1, |acum, dir| {
                        acum + self.solid_disks_line(
                            player_board,
                            opponent_board,
                            *corner as i8,
                            *dir,
                            false,
                        )
                    })
                } else if opponent_board & 1_u64 << corner != 0 {
                    dirs.iter().fold(0, |acum, dir| {
                        acum + self.solid_disks_line(
                            player_board,
                            opponent_board,
                            *corner as i8,
                            *dir,
                            true,
                        )
                    })
                } else {
                    0
                }
            })
    }

    fn solid_disks_line(
        &self,
        player_board: u64,
        opponent_board: u64,
        square: i8,
        diff: i8,
        mut count_if_filled: bool,
    ) -> i8 {
        let mut ret = 0;
        let mut extra = 0;
        for i in 1..8 {
            if player_board & 1 << (square + diff * i) != 0 {
                if count_if_filled {
                    extra += 1;
                } else {
                    ret += 1;
                }
            } else if opponent_board & 1 << (square + diff * i) != 0 {
                count_if_filled = true;
            } else {
                return ret;
            }
        }
        ret + extra
    }

    fn empty_score(&self, color: Color) -> i16 {
        let flippables = self.flippable_squares(color);
        if flippables == 0 {
            -5000
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
    fn get_weight(&self, weight: [i16; 3]) -> i16 {
        let count = self.empty_squares_count();
        let idx = if count > 40 {
            0
        } else if count > 20 {
            1
        } else {
            2
        };
        weight[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::square::Square;

    #[test]
    fn raw_score() {
        let board = Board::initial();
        assert_eq!(board.raw_score(Color::Dark), 0);
    }

    #[test]
    fn mountain_score() {
        let board = Board {
            dark: 0x7e3d81818181817e,
            light: 0,
        };
        assert_eq!(
            board.mountain_score(Color::Dark),
            board.get_weight(Board::MOUNTAIN_WEIGHT) * 2
                + board.get_weight(Board::PURE_MOUNTAIN_WEIGHT) * 1
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

        let board = Board {
            dark: 0x0000e83c465c3c7e,
            light: 0x008000c0b8a0c080,
        };
        assert_eq!(board.solid_disks_count(Color::Light), 5);

        let board = Board {
            dark: 0x0000783c465c3cee,
            light: 0x000080c0b8a0c000,
        };
        assert_eq!(board.solid_disks_count(Color::Dark), 3);
        assert_eq!(board.solid_disks_count(Color::Light), 0);

        // TODO: fix this case

        //let board = Board {
        //dark: 0x0000e83c465c3c7e,
        //light: 0x808000c0b8a0c080,
        //};
        //board.print();
        //assert_eq!(board.solid_disks_count(Color::Light), 7);

        //let board = board.flip(Square::from_str("A1").unwrap(), Color::Light);
        //board.print();
        //assert_eq!(board.solid_disks_count(Color::Light), 13);
    }

    #[test]
    fn openness_of_square() {
        let board = Board::initial();
        let d4 = Square::from_str("D4").unwrap().to_uint();
        let d3 = Square::from_str("D3").unwrap().to_uint();
        assert_eq!(board.openness_of_square(d4), 5);
        assert_eq!(
            board.openness_score(d3, Color::Dark),
            5 * board.get_weight(Board::OPENNESS_WEIGHT)
        );

        let board = Board {
            dark: 0x000014f840200000,
            light: 0x0000200438181000,
        };
        let f2 = Square::from_str("F2").unwrap().to_uint();
        assert_eq!(
            board.openness_score(f2, Color::Light),
            3 * board.get_weight(Board::OPENNESS_WEIGHT)
        );
        let d6 = Square::from_str("D6").unwrap().to_uint();
        assert_eq!(
            board.openness_score(d6, Color::Light),
            7 * board.get_weight(Board::OPENNESS_WEIGHT)
        );
        let g6 = Square::from_str("G6").unwrap().to_uint();
        assert_eq!(
            board.openness_score(g6, Color::Light),
            1 * board.get_weight(Board::OPENNESS_WEIGHT)
        );
    }
}
