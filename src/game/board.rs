use crate::game::common::{Color, Player};
use crate::game::util::{clz, pos_to_shift_amount, pos_to_uint};

pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub struct Board {
    pub white: u64,
    pub black: u64,
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

    pub fn flippable_cells(&self, color: Color) -> u64 {
        let (target, other_target) = self.target_boards(color);

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

    pub fn flipped_board(&self, pos: Position, color: Color) -> u64 {
        let x = 0xffffffffffffffffu64;
        let yzw = 0x7e7e7e7e7e7e7e7eu64;
        let pos_uint = pos_to_uint(&pos);
        let (target, other_target) = self.target_boards(color);

        let mask_x = 0x0080808080808080u64 >> (63 - pos_uint);
        let mask_y = 0x7F00000000000000u64 >> (63 - pos_uint);
        let mask_z = 0x0102040810204000u64 >> (63 - pos_uint);
        let mask_w = 0x0040201008040201u64 >> (63 - pos_uint);

        let mut outflank_x = 0x8000000000000000u64 >> clz(!x & mask_x) & self.white;
        let mut outflank_y = 0x8000000000000000u64 >> clz(!yzw & mask_y) & self.white;
        let mut outflank_z = 0x8000000000000000u64 >> clz(!yzw & mask_z) & self.white;
        let mut outflank_w = 0x8000000000000000u64 >> clz(!yzw & mask_w) & self.white;

        let mut flipped_x = (((-(outflank_x as i64)) * 2) as u64) & mask_x;
        let mut flipped_y = (((-(outflank_y as i64)) * 2) as u64) & mask_y;
        let mut flipped_z = (((-(outflank_z as i64)) * 2) as u64) & mask_z;
        let mut flipped_w = (((-(outflank_w as i64)) * 2) as u64) & mask_w;

        let mask_x = 0x0101010101010100u64 << pos_uint;
        let mask_y = 0x00000000000000feu64 << pos_uint;
        let mask_z = 0x0002040810204080u64 << pos_uint;
        let mask_w = 0x8040201008040200u64 << pos_uint;

        outflank_x = mask_x & ((x | !mask_x) + 1) & target;
        outflank_y = mask_y & ((yzw | !mask_y) + 1) & target;
        outflank_z = mask_z & ((yzw | !mask_z) + 1) & target;
        outflank_w = mask_w & ((yzw | !mask_w) + 1) & target;

        outflank_x = ((outflank_x as i64) - ((outflank_x != 0) as i64)) as u64;
        outflank_y = ((outflank_y as i64) - ((outflank_y != 0) as i64)) as u64;
        outflank_z = ((outflank_z as i64) - ((outflank_z != 0) as i64)) as u64;
        outflank_w = ((outflank_w as i64) - ((outflank_w != 0) as i64)) as u64;

        flipped_x = flipped_x | (outflank_x & mask_x);
        flipped_y = flipped_y | (outflank_y & mask_y);
        flipped_z = flipped_z | (outflank_z & mask_z);
        flipped_w = flipped_w | (outflank_w & mask_w);

        flipped_x | flipped_y | flipped_z | flipped_w
    }

    fn target_boards(&self, color: Color) -> (u64, u64) {
        match color {
            Color::Black => (self.black, self.white),
            Color::White => (self.white, self.black),
        }
    }
}
