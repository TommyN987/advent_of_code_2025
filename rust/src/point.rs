use std::ops::Add;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    const NORTH: Self = Self { x: 0, y: -1 };
    const SOUTH: Self = Self { x: 0, y: 1 };
    const EAST: Self = Self { x: 1, y: 0 };
    const WEST: Self = Self { x: -1, y: 0 };

    pub fn neighbors(self) -> [Self; 8] {
        [
            self + Self::NORTH + Self::WEST,
            self + Self::NORTH,
            self + Self::NORTH + Self::EAST,
            self + Self::EAST,
            self + Self::SOUTH + Self::EAST,
            self + Self::SOUTH,
            self + Self::SOUTH + Self::WEST,
            self + Self::WEST,
        ]
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
