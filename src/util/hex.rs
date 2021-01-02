use std::ops::{Add, AddAssign};

// Hex grid with axial coordinates

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HexCoord(pub i32, pub i32);

impl Add for HexCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for HexCoord {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1);
    }
}

impl HexCoord {
    pub fn direction_pointy(dir_str: &str) -> Option<HexCoord> {
        match dir_str {
            "e" => Some(HexCoord(1, 0)),
            "se" => Some(HexCoord(0, 1)),
            "sw" => Some(HexCoord(-1, 1)),
            "w" => Some(HexCoord(-1, 0)),
            "nw" => Some(HexCoord(0, -1)),
            "ne" => Some(HexCoord(1, -1)),
            _ => None,
        }
    }

    pub fn direction_flat(dir_str: &str) -> Option<HexCoord> {
        match dir_str {
            "se" => Some(HexCoord(1, 0)),
            "s" => Some(HexCoord(0, 1)),
            "sw" => Some(HexCoord(-1, 1)),
            "nw" => Some(HexCoord(-1, 0)),
            "n" => Some(HexCoord(0, -1)),
            "ne" => Some(HexCoord(1, -1)),
            _ => None,
        }
    }

    pub fn dist(self, other: &HexCoord) -> u32 {
        ((self.0 - other.0).abs() as u32
            + (self.0 + self.1 - other.0 - other.1).abs() as u32
            + (self.1 - other.1).abs() as u32)
            / 2
    }
}
