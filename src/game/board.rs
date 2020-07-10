use crate::game::common::{Color, Player};
use crate::game::util::pos_to_shift_amount;

pub struct Board {
    white: u64,
    black: u64,
}

impl Board {
    pub fn initial(player: &Player) -> Board {
        Board {
            white: 1u64 << pos_to_shift_amount("D4").unwrap()
                | 1u64 << pos_to_shift_amount("E5").unwrap(),
            black: 1u64 << pos_to_shift_amount("D5").unwrap()
                | 1u64 << pos_to_shift_amount("E4").unwrap(),
        }
    }

    pub fn flippable_player_cells(&self, color: Color) -> u64 {
        let (target, other_target) = match color {
            Color::Black => (self.black, self.white),
            Color::White => (self.white, self.black),
        };
        let horizontal_watcher = other_target & 0x7e7e7e7e7e7e7e7e;
        let vertical_watcher = other_target & 0x00ffffffffffff00;
        let sides_watcher = other_target & 0x007e7e7e7e7e7e00;
        let blank_cells = !(target | other_target);

        // one can flip atmost 6 disks.
        // opening for loops for speed up

        // west
        let mut tmp = horizontal_watcher & (target << 1);
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        let legal_west = blank_cells & tmp << 1;

        // east
        let mut tmp = horizontal_watcher & target >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        let legal_east = blank_cells & tmp >> 1;

        // top
        let mut tmp = vertical_watcher & target << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        let legal_north = blank_cells & tmp << 8;

        // bottom
        let mut tmp = vertical_watcher & target >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        let legal_south = blank_cells & tmp >> 8;

        // north west
        let mut tmp = sides_watcher & target << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        let legal_north_west = blank_cells & tmp << 9;

        // north east
        let mut tmp = sides_watcher & target << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        let legal_north_east = blank_cells & tmp << 7;

        // south west
        let mut tmp = sides_watcher & target >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        let legal_south_west = blank_cells & tmp >> 7;

        // south east
        let mut tmp = sides_watcher & target >> 9;
        tmp |= sides_watcher & tmp >> 9;
        tmp |= sides_watcher & tmp >> 9;
        tmp |= sides_watcher & tmp >> 9;
        tmp |= sides_watcher & tmp >> 9;
        tmp |= sides_watcher & tmp >> 9;
        let legal_south_east = blank_cells & tmp >> 9;

        legal_west
            | legal_east
            | legal_north
            | legal_south
            | legal_north_west
            | legal_north_east
            | legal_south_west
            | legal_south_east
    }

    // NEXT: fn flip()
}
