//! A simple 2D coordinate type.
//! 
//! This module provides a simple 2D coordinate type that can be used to represent points on a
//! grid. It is intended to be used in conjunction with the `Grid` type, which provides a 2D grid
//! data structure.
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate(pub isize, pub isize);

impl Coordinate {
    #[must_use]
    pub const fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub const fn manhattan_distance(self, other: Self) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }
}

impl From<(isize, isize)> for Coordinate {
    fn from((x, y): (isize, isize)) -> Self {
        Self(x, y)
    }
}

impl From<&(isize, isize)> for Coordinate {
    fn from(&(x, y): &(isize, isize)) -> Self {
        Self(x, y)
    }
}

impl From<Coordinate> for (isize, isize) {
    fn from(coord: Coordinate) -> Self {
        (coord.0, coord.1)
    }
}

impl From<&Coordinate> for (isize, isize) {
    fn from(coord: &Coordinate) -> Self {
        (coord.0, coord.1)
    }
}

// =================================================================================================
// Adding

impl<T> Add<T> for Coordinate
where
    T: Into<(isize, isize)>,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let other = other.into();
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl<T> AddAssign<T> for Coordinate
where
    T: Into<(isize, isize)>,
{
    fn add_assign(&mut self, other: T) {
        let other = other.into();
        self.0 += other.0;
        self.1 += other.1;
    }
}

// =================================================================================================
// Subtraction

impl<T> Sub<T> for Coordinate
where
    T: Into<(isize, isize)>,
{
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        let other = other.into();
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl<T> SubAssign<T> for Coordinate
where
    T: Into<(isize, isize)>,
{
    fn sub_assign(&mut self, other: T) {
        let other = other.into();
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

// =================================================================================================
// Multiplication

impl Mul<isize> for Coordinate {
    type Output = Self;

    fn mul(self, other: isize) -> Self::Output {
        Self(self.0 * other, self.1 * other)
    }
}

impl MulAssign<isize> for Coordinate {
    fn mul_assign(&mut self, other: isize) {
        self.0 *= other;
        self.1 *= other;
    }
}

// =================================================================================================
// Multiplication

impl Mul<usize> for Coordinate {
    type Output = Self;

    fn mul(self, other: usize) -> Self::Output {
        self * other as isize
    }
}

impl MulAssign<usize> for Coordinate {
    fn mul_assign(&mut self, other: usize) {
        self.0 *= other as isize;
        self.1 *= other as isize;
    }
}

// =================================================================================================
// Negation

impl Neg for Coordinate {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}
