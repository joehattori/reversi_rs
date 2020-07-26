#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    // x and y are both 0 indexed
    pub x: u8,
    pub y: u8,
}

impl Square {
    pub fn from_str(s: &str) -> Result<Square, &str> {
        let mut chars = s.chars();
        let x = match chars.next() {
            Some(c) => c as u8 - 'A' as u8,
            None => return Err("While parsing Square: invalid square."),
        };
        let y = match chars.next() {
            Some(c) => match c.to_digit(10) {
                Some(n) => n as u8 - 1,
                None => return Err("While parsing Square: invalid square."),
            },
            None => return Err("While parsing Square: invalid square."),
        };
        Ok(Square { x, y })
    }

    pub fn from_uint(i: u8) -> Square {
        Square { x: i % 8, y: i / 8 }
    }

    pub fn to_uint(&self) -> u8 {
        self.x + self.y * 8
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", (self.x + 'A' as u8) as char, self.y + 1)
    }
}
