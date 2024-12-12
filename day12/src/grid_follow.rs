use std::collections::{HashSet, VecDeque};

use aoc_grid::{Coordinate, Direction, Grid};

pub struct GridFillFollower<'a, T, P> {
    grid: &'a Grid<T>,
    todo: VecDeque<Coordinate>,
    visited: HashSet<Coordinate>,
    predicate: P,
}

pub trait FollowArea<'a, T> {
    fn follow_area<P>(&'a self, start: &Coordinate, predicate: P) -> GridFillFollower<'a, T, P>
    where
        P: FnMut(&T) -> bool;
}

impl<'a, T> FollowArea<'a, T> for Grid<T> {
    fn follow_area<P>(&'a self, start: &Coordinate, predicate: P) -> GridFillFollower<'a, T, P>
    where
        P: FnMut(&T) -> bool,
    {
        let mut todo = VecDeque::new();
        todo.push_back(*start);

        GridFillFollower {
            grid: self,
            todo,
            visited: HashSet::new(),
            predicate,
        }
    }
}

impl<T, P> Iterator for GridFillFollower<'_, T, P>
where
    P: FnMut(&T) -> bool,
{
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.todo.pop_front()?;

            if !self.visited.insert(current) {
                // Coordinate already visited
                continue;
            }

            for direction in Direction::CARDINAL_4 {
                let neighbor = current + &direction;
                if self.grid.contains(neighbor) {
                    let neighbor_value = &self.grid[&neighbor];
                    if (self.predicate)(neighbor_value) {
                        self.todo.push_back(neighbor);
                    }
                }
            }

            return Some(current);
        }
    }
}
