#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Square {
    // x and y are both 0 indexed
    pub x: u8,
    pub y: u8,
}

impl Square {
    pub fn from_str(s: &str) -> Result<Self, &str> {
        let mut chars = s.chars();
        let x = match chars.next() {
            Some(c) => {
                let base = if c.is_uppercase() { 'A' } else { 'a' };
                c as u8 - base as u8
            }
            None => return Err("While parsing Square: invalid square."),
        };
        let y = match chars.next() {
            Some(c) => match c.to_digit(10) {
                Some(n) => n as u8 - 1,
                None => return Err("While parsing Square: invalid square."),
            },
            None => return Err("While parsing Square: invalid square."),
        };
        Ok(Self { x, y })
    }

    pub fn from_uint(i: u8) -> Self {
        Self { x: i % 8, y: i / 8 }
    }

    pub fn to_uint(&self) -> u8 {
        self.x + self.y * 8
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", (self.x + 'A' as u8) as char, self.y + 1)
    }

    pub fn mirror(&self) -> Self {
        Self {
            x: self.x,
            y: 7 - self.y,
        }
    }

    pub fn rotate_90(&self) -> Self {
        Self {
            x: 7 - self.y,
            y: self.x,
        }
    }

    pub fn rotate_180(&self) -> Self {
        self.rotate_90().rotate_90()
    }

    pub fn rotate_270(&self) -> Self {
        self.rotate_180().rotate_90()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_90() {
        let s = Square::from_str("A6").unwrap();
        assert_eq!(s.rotate_90(), Square::from_str("C1").unwrap());

        let s = Square::from_str("B4").unwrap();
        assert_eq!(s.rotate_90(), Square::from_str("E2").unwrap());

        let s = Square::from_str("E3").unwrap();
        assert_eq!(s.rotate_90(), Square::from_str("F5").unwrap());
    }
}
