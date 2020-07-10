use crate::game::board::Position;

// ex: pos_to_shift_amount("C4") -> 26
pub fn pos_to_shift_amount(pos: &str) -> Result<u8, &str> {
    let mut chars = pos.chars();
    let alphabet_diff = match chars.next() {
        Some(c) => c as u8 - 'A' as u8,
        None => return Err("While parsing pos: invalid position."),
    };
    let number = match chars.next() {
        Some(c) => match c.to_digit(10) {
            Some(n) => n as u8,
            None => return Err("While parsing pos: invalid position."),
        },
        None => return Err("While parsing pos: invalid position."),
    };
    Ok((alphabet_diff + 1) + (number - 1) * 8)
}

pub fn pos_to_uint(pos: &Position) -> u8 {
    pos.x + pos.y * 8
}

pub fn clz(x: u64) -> u8 {
    x.leading_zeros() as u8
}
