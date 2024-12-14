use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub i64, pub i64);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i64> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i64) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
}

impl Coord {
    pub const UP: Coord = Coord(0, -1);
    pub const RIGHT: Coord = Coord(1, 0);
    pub const DOWN: Coord = Coord(0, 1);
    pub const LEFT: Coord = Coord(-1, 0);

    pub fn up(&self) -> Self {
        *self + Self::UP
    }

    pub fn right(&self) -> Self {
        *self + Self::RIGHT
    }

    pub fn down(&self) -> Self {
        *self + Self::DOWN
    }

    pub fn left(&self) -> Self {
        *self + Self::LEFT
    }
}
