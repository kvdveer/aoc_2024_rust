use crate::coordinate::Coordinate;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

/// A cardinal direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    UpLeft = 5,
    Left = 6,
    DownLeft = 7,
}

impl Direction {
    // The four cardinal directions, not including diagonals.
    pub const CARDINAL_4: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    // The eight cardinal directions, including diagonals.
    pub const CARDINAL_8: [Self; 8] = [
        Self::Up,
        Self::UpRight,
        Self::Right,
        Self::DownRight,
        Self::Down,
        Self::DownLeft,
        Self::Left,
        Self::UpLeft,
    ];

    #[must_use]
    pub const fn counter_clockwise_4(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,

            Self::UpRight => Self::UpLeft,
            Self::UpLeft => Self::DownLeft,
            Self::DownLeft => Self::DownRight,
            Self::DownRight => Self::UpRight,
        }
    }

    #[must_use]
    pub const fn counter_clockwise_8(self) -> Self {
        match self {
            Self::Up => Self::UpLeft,
            Self::UpLeft => Self::Left,
            Self::Left => Self::DownLeft,
            Self::DownLeft => Self::Down,
            Self::Down => Self::DownRight,
            Self::DownRight => Self::Right,
            Self::Right => Self::UpRight,
            Self::UpRight => Self::Up,
        }
    }
    #[must_use]
    pub const fn clockwise_8(self) -> Self {
        match self {
            Self::Up => Self::UpRight,
            Self::UpRight => Self::Right,
            Self::Right => Self::DownRight,
            Self::DownRight => Self::Down,
            Self::Down => Self::DownLeft,
            Self::DownLeft => Self::Left,
            Self::Left => Self::UpLeft,
            Self::UpLeft => Self::Up,
        }
    }

    #[must_use]
    pub const fn clockwise_4(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,

            Self::UpRight => Self::DownRight,
            Self::DownRight => Self::DownLeft,
            Self::DownLeft => Self::UpLeft,
            Self::UpLeft => Self::UpRight,
        }
    }

    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,

            Self::UpRight => Self::DownLeft,
            Self::DownLeft => Self::UpRight,
            Self::UpLeft => Self::DownRight,
            Self::DownRight => Self::UpLeft,
        }
    }
}
impl From<Direction> for Coordinate {
    fn from(dir: Direction) -> Self {
        Coordinate::from(&dir)
    }
}

impl From<&Direction> for Coordinate {
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::Up => Coordinate(0, -1),
            Direction::Down => Coordinate(0, 1),
            Direction::Left => Coordinate(-1, 0),
            Direction::Right => Coordinate(1, 0),

            Direction::UpRight => Coordinate(1, -1),
            Direction::UpLeft => Coordinate(-1, -1),
            Direction::DownRight => Coordinate(1, 1),
            Direction::DownLeft => Coordinate(-1, 1),
        }
    }
}

impl Add<&Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &Direction) -> Self::Output {
        let rhs: Coordinate = rhs.into();
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<&Coordinate> for Direction {
    type Output = Coordinate;

    fn add(self, rhs: &Coordinate) -> Self::Output {
        let lhs: Coordinate = (&self).into();
        Coordinate(lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}

impl AddAssign<&Direction> for Coordinate {
    fn add_assign(&mut self, rhs: &Direction) {
        let rhs: Coordinate = rhs.into();
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<&Coordinate> for Direction {
    type Output = Coordinate;

    fn sub(self, rhs: &Coordinate) -> Self::Output {
        let lhs: Coordinate = (&self).into();
        Coordinate(lhs.0 - rhs.0, lhs.1 - rhs.1)
    }
}

impl Sub<&Direction> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: &Direction) -> Self::Output {
        let rhs: Coordinate = (rhs).into();
        Coordinate(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign<&Direction> for Coordinate {
    fn sub_assign(&mut self, rhs: &Direction) {
        let rhs: Coordinate = (rhs).into();
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.opposite()
    }
}

impl Mul<isize> for &Direction {
    type Output = Coordinate;

    fn mul(self, rhs: isize) -> Self::Output {
        let lhs: Coordinate = (self).into();
        Coordinate::new(lhs.0 * rhs, lhs.1 * rhs)
    }
}
impl Mul<usize> for &Direction {
    type Output = Coordinate;

    fn mul(self, rhs: usize) -> Self::Output {
        let lhs: Coordinate = (self).into();
        Coordinate::new(lhs.0 * rhs as isize, lhs.1 * rhs as isize)
    }
}

impl From<Direction> for char {
    fn from(value: Direction) -> char {
        match value {
            Direction::Up => '🡑',
            Direction::Down => '🡓',
            Direction::Left => '🡐',
            Direction::Right => '🡒',

            Direction::UpRight => '↗',
            Direction::UpLeft => '↖',
            Direction::DownRight => '↘',
            Direction::DownLeft => '↙',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_cast() {
        assert_eq!(Coordinate(0, -1), Direction::Up.into());
        assert_eq!(Coordinate(0, 1), Direction::Down.into());
        assert_eq!(Coordinate(-1, 0), Direction::Left.into());
        assert_eq!(Coordinate(1, 0), Direction::Right.into());

        assert_eq!(Coordinate(1, -1), Direction::UpRight.into());
        assert_eq!(Coordinate(-1, -1), Direction::UpLeft.into());
        assert_eq!(Coordinate(1, 1), Direction::DownRight.into());
        assert_eq!(Coordinate(-1, 1), Direction::DownLeft.into());
    }
    #[test]
    fn test_rotation() {
        assert_eq!(Direction::Up, Direction::Left.clockwise_4());
        assert_eq!(Direction::UpLeft, Direction::Left.clockwise_8());
    }

    #[test]
    fn test_math() {
        assert_eq!(Direction::Up + &Coordinate(1,2), Coordinate(1,1));
    }
}
