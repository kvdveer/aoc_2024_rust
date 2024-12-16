//! Grid module
//! A grid is a 2D array of cells, where each cell can be accessed by its x and y coordinates.

use std::{
    fmt::{self},
    ops::{Index, IndexMut},
};

use crate::grid_index::GridIndex;
use crate::Coordinate;

#[derive(PartialEq, Clone, Hash, Eq)]
pub struct Grid<T> {
    stride: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    // Creates a new Grid with the given width and height, with each cell initialized to the default value of T.
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        let cells = vec![T::default(); width * height];
        Self {
            stride: width,
            cells,
        }
    }

    // Creates a new Grid with the given width and height, with each cell initialized to the given value.
    // The provided iterator must yield exactly width * height elements, and be in row-major order.
    pub fn new_from_iter<InputItem: Into<T>>(
        width: usize,
        height: usize,
        cells: impl Iterator<Item = InputItem>,
    ) -> Self {
        let cells: Vec<T> = cells.map(|e| e.into()).collect();
        assert_eq!(cells.len(), width * height);
        Self {
            stride: width,
            cells,
        }
    }

    // Height of the grid
    pub fn height(&self) -> usize {
        self.cells.len() / self.stride
    }

    // Width of the grid
    pub const fn width(&self) -> usize {
        self.stride
    }

    // Wraps a coordinate to be inside the grid boundaries.
    pub fn wrap<U: GridIndex>(&self, pos: U) -> Coordinate {
        let w = self.width() as isize;
        let h = self.height() as isize;

        let x = pos.x().rem_euclid(w);
        let y = pos.y().rem_euclid(h);
        Coordinate::new(x, y)
    }

    // Linearizes a 2D coordinate into a 1D index, used for the underlying array.
    fn offset<U: GridIndex>(&self, pos: U) -> Option<usize> {
        let x = pos.x();
        let y = pos.y();
        if x >= 0 && x < self.width() as isize && y >= 0 && y < self.height() as isize {
            Some((y * self.stride as isize + x) as usize)
        } else {
            None
        }
    }

    // Returns a new grid with the same dimensions as self, but with each cell mapped to a new value.
    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Grid<U> {
        let cells = self.cells.iter().map(f).collect();
        Grid {
            stride: self.stride,
            cells,
        }
    }

    // Iterates over the cells of the grid.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.cells.iter()
    }

    // Merges two grids together, cell by cell, with tuples per cell.
    pub fn zip<U, V>(&self, other: &Grid<U>, f: impl Fn((&T, &U)) -> V) -> Grid<V> {
        Grid::<V>::new_from_iter(
            self.width(),
            self.height(),
            self.iter().zip(other.iter()).map(|(a, b)| f((a, b))),
        )
    }

    // Retrieves a value from the grid, if the provided position is within the grid boundaries.
    pub fn get<U: GridIndex + Clone>(&self, pos: U) -> Option<&T> {
        Some(&self.cells[self.offset(pos)?])
    }

    // Retrieves a mutable value from the grid, if the provided position is within the grid boundaries.
    pub fn get_mut<U: GridIndex + Clone>(&mut self, pos: U) -> Option<&mut T> {
        let offset = self.offset(pos)?;
        Some(&mut self.cells[offset])
    }

    // Iterates over the entries of the grid.
    pub fn iter_coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        self.cells.iter().enumerate().map(move |(i, _)| {
            let x = i % self.stride;
            let y = i / self.stride;
            Coordinate(x as isize, y as isize)
        })
    }
    // Iterates over the entries of the grid.
    pub fn iter_pairs(&self) -> impl Iterator<Item = (Coordinate, &T)> + '_ {
        self.cells.iter().enumerate().map(move |(i, _)| {
            let x = i % self.stride;
            let y = i / self.stride;
            (Coordinate(x as isize, y as isize), &self.cells[i])
        })
    }

    pub fn contains<U: GridIndex>(&self, pos: U) -> bool {
        let x = pos.x();
        let y = pos.y();
        x >= 0 && (x < self.width() as isize) && y >= 0 && y < (self.height() as isize)
    }

    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.cells.fill(value);
    }
}

// Indexing into the grid. This requires the grid position to be valid
impl<T, U> Index<U> for Grid<T>
where
    U: GridIndex + Clone,
{
    type Output = T;

    fn index(&self, position: U) -> &Self::Output {
        self.get(position).expect("Index outside grid")
    }
}

// Mutable indexing into the grid. This requires the grid position to be valid
impl<T, U> IndexMut<U> for Grid<T>
where
    U: GridIndex + Clone,
{
    fn index_mut(&mut self, position: U) -> &mut Self::Output {
        self.get_mut(position).expect("Index outside grid")
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    // Returns a new grid with the same dimensions as self, with the rows and columns swapped.
    pub fn transpose(&self) -> Self {
        let new_stride = self.height();
        let mut cells = Vec::<T>::with_capacity(self.cells.len());
        for x in 0..self.width() {
            for y in 0..self.height() {
                cells.push(self.cells[y * self.stride + x].clone());
            }
        }
        Self {
            stride: new_stride,
            cells,
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    // Creates a new Grid from a nested vectors of cells.
    fn from(v: Vec<Vec<T>>) -> Self {
        if v.is_empty() {
            return Self {
                stride: 0,
                cells: Vec::new(),
            };
        }

        if v[0].is_empty() {
            return Self {
                stride: 0,
                cells: Vec::new(),
            };
        }

        let stride = v[0].len();

        let mut cells = Vec::<T>::with_capacity(v.len() * stride);
        cells.extend(
            v.into_iter()
                .inspect(|a| assert!(a.len() == stride, "Rows must be of equal length"))
                .flatten(),
        );

        Self { stride, cells }
    }
}

impl<T> fmt::Display for Grid<T>
where
    char: From<T>,
    T: Copy,
{
    // If the cells can be converted to characters, display the grid as a 2D array of characters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cell = &self.cells[self.offset((x, y)).unwrap()];
                let c: char = char::from(*cell);
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> fmt::Debug for Grid<T>
where
    char: From<T>,
    T: Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = Grid::from(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
    }

    #[test]
    fn test_grid_index() {
        let grid = Grid::from(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        assert_eq!(grid[(0, 0)], 'a');
        assert_eq!(grid[(1, 0)], 'b');
        assert_eq!(grid[(2, 0)], 'c');
        assert_eq!(grid[(0, 1)], 'd');
        assert_eq!(grid[(1, 1)], 'e');
        assert_eq!(grid[(2, 1)], 'f');
        assert_eq!(grid[(0, 2)], 'g');
        assert_eq!(grid[(1, 2)], 'h');
        assert_eq!(grid[(2, 2)], 'i');
    }

    #[test]
    fn test_grid_get() {
        let grid = Grid::from(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);
        assert_eq!(grid.get((0, 0)), Some(&'a'));
        assert_eq!(grid.get((1, 0)), Some(&'b'));
        assert_eq!(grid.get((2, 0)), Some(&'c'));
        assert_eq!(grid.get((0, 1)), Some(&'d'));
        assert_eq!(grid.get((1, 1)), Some(&'e'));
        assert_eq!(grid.get((2, 1)), Some(&'f'));
        assert_eq!(grid.get((0, 2)), Some(&'g'));
        assert_eq!(grid.get((1, 2)), Some(&'h'));
        assert_eq!(grid.get((2, 2)), Some(&'i'));

        assert_eq!(grid.get((3, 0)), None);
        assert_eq!(grid.get((0, 3)), None);
        assert_eq!(grid.get((3, 3)), None);

        assert_eq!(grid.get((0, -1)), None);
        assert_eq!(grid.get((-1, 0)), None);
        assert_eq!(grid.get((-1, -1)), None);
    }

    #[test]
    fn test_grid_entry() {
        let grid = Grid::from(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);
        assert_eq!(grid.get((0, 0)), Some(&'a'));
        assert_eq!(grid.get((1, 0)), Some(&'b'));
        assert_eq!(grid.get((4, 0)), None);
    }

    #[test]
    fn test_grid_contains() {
        let grid = Grid::from(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);
        assert!(grid.contains((0, 0)));
        assert!(!grid.contains((-1, 0)));
        assert!(!grid.contains((3, 0)));
        assert!(!grid.contains((0, -1)));
        assert!(!grid.contains((0, 3)));
    }

    #[test]
    fn test_transpose() {
        let grid = Grid::from(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);

        let transposed = grid.transpose();

        assert_eq!(transposed.width(), 2);
        assert_eq!(transposed.height(), 3);

        assert_eq!(transposed[(0, 0)], 'a');
        assert_eq!(transposed[(0, 1)], 'b');
        assert_eq!(transposed[(0, 2)], 'c');
        assert_eq!(transposed[(1, 0)], 'd');
        assert_eq!(transposed[(1, 1)], 'e');
        assert_eq!(transposed[(1, 2)], 'f');
    }
}
