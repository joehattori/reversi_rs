use crate::game::common::Color;
use crate::game::util::clz;

pub struct Position {
    // x and y are both 0 indexed
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn from_str(s: &str) -> Result<Position, &str> {
        let mut chars = s.chars();
        let x = match chars.next() {
            Some(c) => c as u8 - 'A' as u8,
            None => return Err("While parsing pos: invalid position."),
        };
        let y = match chars.next() {
            Some(c) => match c.to_digit(10) {
                Some(n) => n as u8 - 1,
                None => return Err("While parsing pos: invalid position."),
            },
            None => return Err("While parsing pos: invalid position."),
        };
        Ok(Position { x, y })
    }

    pub fn to_uint(&self) -> u8 {
        self.x + self.y * 8
    }

    pub fn to_cell_string(&self) -> String {
        format!("{}{}", (self.x + 'A' as u8) as char, self.y + 1)
    }
}

#[derive(Copy, Clone)]
pub struct Board {
    pub white: u64,
    pub black: u64,
}

impl Board {
    pub fn initial() -> Board {
        Board {
            white: 1u64 << Position::from_str("D4").unwrap().to_uint()
                | 1u64 << Position::from_str("E5").unwrap().to_uint(),
            black: 1u64 << Position::from_str("D5").unwrap().to_uint()
                | 1u64 << Position::from_str("E4").unwrap().to_uint(),
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

            if self.black & 1 << i != 0 {
                print!("x ");
            } else if self.white & 1 << i != 0 {
                print!("o ");
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }

    pub fn flip(&self, pos: &Position, color: Color) -> Board {
        let flipped = self.flipped_cells(&pos, color);
        match color {
            Color::Black => Board {
                black: self.black | 1u64 << pos.to_uint() | flipped,
                white: self.white & !flipped,
            },
            Color::White => Board {
                black: self.black & !flipped,
                white: self.white | 1u64 << pos.to_uint() | flipped,
            },
        }
    }

    pub fn flippable_cells(&self, color: Color) -> u64 {
        let (target_board, other_board) = self.target_boards(color);

        let horizontal_watcher = other_board & 0x7e7e7e7e7e7e7e7e;
        let vertical_watcher = other_board & 0x00ffffffffffff00;
        let sides_watcher = other_board & 0x007e7e7e7e7e7e00;
        let blank_cells = !(target_board | other_board);

        // one can flip atmost 6 disks.
        // opening for loops for speed up

        // west
        let mut tmp = horizontal_watcher & (target_board << 1);
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        tmp |= horizontal_watcher & tmp << 1;
        let legal_west = blank_cells & tmp << 1;

        // east
        let mut tmp = horizontal_watcher & target_board >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        tmp |= horizontal_watcher & tmp >> 1;
        let legal_east = blank_cells & tmp >> 1;

        // top
        let mut tmp = vertical_watcher & target_board << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        tmp |= vertical_watcher & tmp << 8;
        let legal_north = blank_cells & tmp << 8;

        // bottom
        let mut tmp = vertical_watcher & target_board >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        tmp |= vertical_watcher & tmp >> 8;
        let legal_south = blank_cells & tmp >> 8;

        // north west
        let mut tmp = sides_watcher & target_board << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        tmp |= sides_watcher & tmp << 9;
        let legal_north_west = blank_cells & tmp << 9;

        // north east
        let mut tmp = sides_watcher & target_board << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        tmp |= sides_watcher & tmp << 7;
        let legal_north_east = blank_cells & tmp << 7;

        // south west
        let mut tmp = sides_watcher & target_board >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        tmp |= sides_watcher & tmp >> 7;
        let legal_south_west = blank_cells & tmp >> 7;

        // south east
        let mut tmp = sides_watcher & target_board >> 9;
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

    pub fn flipped_cells(&self, pos: &Position, color: Color) -> u64 {
        let (target_board, mut other_board) = self.target_boards(color);
        let pos_uint = pos.to_uint();

        let mut ret = 0u64;

        let mut mask = 0x0080808080808080u64 >> (63 - pos_uint);
        let mut outflank = 0x8000000000000000u64 >> clz(!other_board & mask) & target_board;
        let mut flipped = (-(outflank as i128) * 2) as u64 & mask;
        mask = 0x0101010101010100u64 << pos_uint;
        outflank = mask & ((other_board | !mask) as u128 + 1) as u64 & target_board;
        flipped |= (outflank as i128 - ((outflank != 0) as i128)) as u64 & mask;
        ret |= flipped;

        other_board &= 0x7e7e7e7e7e7e7e7eu64;

        let mut mask = 0x7f00000000000000u64 >> (63 - pos_uint);
        let mut outflank = 0x8000000000000000u64 >> clz(!other_board & mask) & target_board;
        let mut flipped = (-(outflank as i128) * 2) as u64 & mask;
        mask = 0x00000000000000feu64 << pos_uint;
        outflank = mask & ((other_board | !mask) as u128 + 1) as u64 & target_board;
        flipped |= (outflank as i128 - ((outflank != 0) as i128)) as u64 & mask;
        ret |= flipped;

        let mut mask = 0x0102040810204000u64 >> (63 - pos_uint);
        let mut outflank = 0x8000000000000000u64 >> clz(!other_board & mask) & target_board;
        let mut flipped = (-(outflank as i128) * 2) as u64 & mask;
        mask = 0x0002040810204080u64 << pos_uint;
        outflank = mask & ((other_board | !mask) as u128 + 1) as u64 & target_board;
        flipped |= (outflank as i128 - ((outflank != 0) as i128)) as u64 & mask;
        ret |= flipped;

        let mut mask = 0x0040201008040201u64 >> (63 - pos_uint);
        let mut outflank = 0x8000000000000000u64 >> clz(!other_board & mask) & target_board;
        let mut flipped = (-(outflank as i128) * 2) as u64 & mask;
        mask = 0x8040201008040200u64 << pos_uint;
        outflank = mask & ((other_board | !mask) as u128 + 1) as u64 & target_board;
        flipped |= (outflank as i128 - ((outflank != 0) as i128)) as u64 & mask;
        ret |= flipped;

        ret
    }

    fn target_boards(&self, color: Color) -> (u64, u64) {
        match color {
            Color::Black => (self.black, self.white),
            Color::White => (self.white, self.black),
        }
    }
}
