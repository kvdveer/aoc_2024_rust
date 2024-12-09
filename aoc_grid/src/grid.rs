//! Grid module
//! A grid is a 2D array of cells, where each cell can be accessed by its x and y coordinates.

use std::{
    fmt::{self},
    ops::{Index, IndexMut},
};

use crate::Coordinate;

#[derive(PartialEq, Clone, Hash, Eq)]
pub struct Grid<T> {
    stride: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    // Creates a new Grid with the given width and height, with each cell initialized to the default value of T.
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn height(&self) -> usize {
        self.cells.len() / self.stride
    }

    // Width of the grid
    #[must_use]
    pub const fn width(&self) -> usize {
        self.stride
    }

    // Wraps a coordinate to be inside the grid boundaries.
    #[must_use]
    pub fn wrap<U: Into<(isize, isize)>>(&self, pos: U) -> Coordinate {
        let w = self.width() as isize;
        let h = self.height() as isize;

        let pos: (isize, isize) = pos.into();
        let mut x = pos.0 % w;
        let mut y = pos.1 % h;
        if x < 0 {
            x += w;
        }
        if y < 0 {
            y += h
        }
        Coordinate::new(x, y)
    }

    // Linearizes a 2D coordinate into a 1D index, used for the underlying array.
    fn offset<U: Into<(isize, isize)>>(&self, pos: U) -> usize {
        let pos: (isize, isize) = pos.into();
        (pos.1 * self.stride as isize + pos.0) as usize
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
    pub fn get<U: Into<(isize, isize)>>(&self, pos: U) -> Option<&T> {
        let pos: (isize, isize) = pos.into();
        if self.coordinate_valid(pos) {
            Some(&self.cells[self.offset(pos)])
        } else {
            None
        }
    }

    // Retrieves a mutable value from the grid, if the provided position is within the grid boundaries.
    pub fn get_mut<U: Into<(isize, isize)>>(&mut self, pos: U) -> Option<&mut T> {
        let pos: (isize, isize) = pos.into();
        if self.coordinate_valid(pos) {
            let offset = self.offset(pos);
            Some(&mut self.cells[offset])
        } else {
            None
        }
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

    pub fn coordinate_valid<U: Into<(isize, isize)>>(&self, pos: U) -> bool {
        let pos: (isize, isize) = pos.into();
        pos.0 >= 0
            && (pos.0 < self.width() as isize)
            && pos.1 >= 0
            && pos.1 < (self.height() as isize)
    }
}

// Indexing into the grid. This requires the grid position to be valid
impl<T, U> Index<U> for Grid<T>
where
    U: Into<(isize, isize)>,
{
    type Output = T;

    fn index(&self, position: U) -> &Self::Output {
        self.get(position).expect("Index outside grid")
    }
}

// Mutable indexing into the grid. This requires the grid position to be valid
impl<T, U> IndexMut<U> for Grid<T>
where
    U: Into<(isize, isize)>,
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
    #[must_use]
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
        writeln!(f)?;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cell = &self.cells[self.offset((x as isize, y as isize))];
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
    fn test_grid_coordinate_valid() {
        let grid = Grid::from(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);
        assert!(grid.coordinate_valid((0, 0)));
        assert!(!grid.coordinate_valid((-1, 0)));
        assert!(!grid.coordinate_valid((3, 0)));
        assert!(!grid.coordinate_valid((0, -1)));
        assert!(!grid.coordinate_valid((0, 3)));
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
