use std::collections::HashSet;

use aoc_grid::{Coordinate, Direction, Grid};

use crate::puzzle_input::{MapElement, PuzzleInput};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Guard {
    position: Coordinate,
    direction: Direction,
}

impl Guard {
    fn new(guard_position: Coordinate, guard_direction: Direction) -> Self {
        Self {
            position: guard_position,
            direction: guard_direction,
        }
    }

    fn step(&mut self, map: &Grid<MapElement>, extra_obstacle: &Coordinate) -> bool {
        let mut next_position = self.position + &self.direction;
        let mut next_element = map.get(next_position);

        while next_element == Some(&MapElement::Obstacle) || next_position == extra_obstacle {
            self.direction = self.direction.clockwise_4();
            next_position = self.position + &self.direction;
            next_element = map.get(next_position);
        }

        self.position = next_position;

        next_element.is_some() // Return if we're still on the map
    }
}

fn does_loop(map: &Grid<MapElement>, mut guard: Guard, extra_obstacle: &Coordinate) -> bool {
    let mut visited = Grid::<Option<Direction>>::new(map.width(), map.height());
    loop {
        if !guard.step(map, extra_obstacle) {
            // Guard walked off the map
            return false;
        }
        if visited[guard.position] == Some(guard.direction) {
            return true;
        }

        visited[guard.position] = Some(guard.direction);
    }
}

pub fn solve(input: &PuzzleInput) -> String {
    let guard_position = input
        .map
        .iter_coordinates()
        .find(|&c| input.map[c] == MapElement::Guard)
        .expect("map should have a guard position");

    // Build a list of locations that the guard has visited in an unaltered map
    let mut guard = Guard::new(guard_position, Direction::Up);
    let mut visited_positions = HashSet::with_capacity(input.map.width() * 10);
    let fake_obstacle = Coordinate::new(-2, -2);
    while guard.step(&input.map, &fake_obstacle) {
        if guard.position != guard_position {
            visited_positions.insert(guard.position);
        }
    }

    visited_positions
        .iter()
        .filter(|coordinate| {
            does_loop(
                &input.map,
                Guard::new(guard_position, Direction::Up),
                coordinate,
            )
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "6")]
    #[case::final_input( include_str!("../input.txt"), "1670")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
