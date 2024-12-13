use aoc_grid::{Coordinate, Direction, Grid};

pub struct GridFillFollower<'a, T, P> {
    grid: &'a Grid<T>,
    todo: Vec<Coordinate>,
    visited: Grid<bool>,
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
        let mut todo = Vec::with_capacity(self.width());
        todo.push(*start);

        GridFillFollower {
            grid: self,
            todo,
            visited: Grid::new(self.width(), self.height()),
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
            let current = self.todo.pop()?;

            if self.visited[current] {
                // Coordinate already visited
                continue;
            }
            self.visited[current] = true;

            for direction in Direction::CARDINAL_4 {
                let neighbor = current + &direction;
                if self.grid.contains(neighbor) {
                    let neighbor_value = &self.grid[&neighbor];
                    if (self.predicate)(neighbor_value) {
                        self.todo.push(neighbor);
                    }
                }
            }

            return Some(current);
        }
    }
}
