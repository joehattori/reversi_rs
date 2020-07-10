use crate::game::common::{Color, Player};
use crate::game::util::pos_to_shift_amount;

pub struct Board {
    player: u64,
    opponent: u64,
}

impl Board {
    pub fn initial(player: &Player) -> Board {
        match player.color {
            Color::Black => Board {
                player: 1u64 << pos_to_shift_amount("D4").unwrap()
                    | 1u64 << pos_to_shift_amount("E5").unwrap(),
                opponent: 1u64 << pos_to_shift_amount("D5").unwrap()
                    | 1u64 << pos_to_shift_amount("E4").unwrap(),
            },
            Color::White => Board {
                player: 1u64 << pos_to_shift_amount("D5").unwrap()
                    | 1u64 << pos_to_shift_amount("E4").unwrap(),
                opponent: 1u64 << pos_to_shift_amount("D4").unwrap()
                    | 1u64 << pos_to_shift_amount("E5").unwrap(),
            },
        }
    }
}
