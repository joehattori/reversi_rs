use super::board::Board;
use crate::game::base::Color;

impl Board {
    // TODO: polish
    const MOUNTAIN_WEIGHT: [i16; 3] = [30, 20, 10];
    const PURE_MOUNTAIN_WEIGHT: [i16; 3] = [50, 40, 20];
    const WING_WEIGHT: [i16; 3] = [-20, -10, -1];
    const SUB_WING_WEIGHT: [i16; 3] = [-10, -5, -1];
    const CORNER_FLIPPABLE_WEIGHT: [i16; 3] = [100, 80, 40];
    const SOLID_DISK_WEIGHT: [i16; 3] = [10, 10, 10];
    const FLIPPABLE_COUNT_WEIGHT: [i16; 3] = [10, 10, 10];
    const SQUARE_VALUES: [i16; 64] = [
        100, -40, 20, 5, 5, 20, -40, 100, -40, -80, -1, -1, -1, -1, -80, -40, 20, -1, 5, 1, 1, 5,
        -1, 20, 5, -1, 1, 0, 0, 1, -1, 5, 5, -1, 1, 0, 0, 1, -1, 5, 20, -1, 5, 1, 1, 5, -1, 20,
        -40, -80, -1, -1, -1, -1, -80, -40, 100, -40, 20, 5, 5, 20, -40, 100,
    ];

    #[inline]
    pub fn score(&self, color: Color) -> i16 {
        self.raw_score(color)
            + self.flippable_count_score(color)
            + self.mountain_score(color)
            + self.corner_flippable_score(color)
            + self.wing_score(color)
            + self.solid_disks_score(color)
            + self.sub_wing_score(color)
            + self.empty_score(color)
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
    fn flippable_count_score(&self, color: Color) -> i16 {
        let player_flippable = self.flippable_squares(color);
        let opponent_flippable = self.flippable_squares(color.opposite());
        (player_flippable.count_ones() as i16 - 2 * opponent_flippable.count_ones() as i16)
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
        (self.solid_disks_count(color) - 2 * self.solid_disks_count(color.opposite())) as i16
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
                ret + if (player_board | opponent_board) & 1_u64 << corner != 0 {
                    let default = (player_board >> corner & 1) as i8;
                    dirs.iter().fold(default, |acum, dir| {
                        acum + self.solid_disks_line(
                            player_board,
                            opponent_board,
                            *corner as i8,
                            *dir,
                        )
                    })
                } else {
                    0
                }
            })
    }

    fn solid_disks_line(&self, player_board: u64, opponent_board: u64, square: i8, diff: i8) -> i8 {
        let mut ret = 0;
        let mut extra = 0;
        let mut all_same = true;
        for i in 1..8 {
            if player_board & 1 << (square + diff * i) != 0 {
                if all_same {
                    ret += 1;
                } else {
                    extra += 1;
                }
            } else if opponent_board & 1 << (square + diff * i) != 0 {
                all_same = false;
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
        let idx = if count > 30 {
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

    #[test]
    fn mountain_score() {
        let board = Board {
            dark: 0x7e0181818181817e,
            light: 0,
        };
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

        let board = Board {
            dark: 0x0000e83c465c3c7e,
            light: 0x008000c0b8a0c080,
        };
        assert_eq!(board.solid_disks_count(Color::Light), 5);

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
}
