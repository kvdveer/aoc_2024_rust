use aoc_grid::{Coordinate, Direction, Grid};

use crate::puzzle_input::{MapElement, PuzzleInput};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
enum Visited {
    #[default]
    NotVisited,
    Visited,
}

struct MapState {
    map: Grid<MapElement>,
    guard_position: Coordinate,
    guard_direction: Direction,
}

impl MapState {
    fn new(base_map: &Grid<MapElement>) -> Self {
        Self {
            // Remove the guard from the map
            map: base_map.map(|&c| match c {
                MapElement::Guard => MapElement::Empty,
                _ => c,
            }),

            // Look up the guard position
            guard_position: base_map
                .iter_coordinates()
                .find(|&c| base_map[c] == MapElement::Guard)
                .expect("map should have a guard position"),

            // Guard always starts facing up
            guard_direction: Direction::Up,
        }
    }

    fn does_loop(&self, extra_obstacle: Coordinate) -> bool {
        let mut guard_position = self.guard_position;
        let mut guard_direction = self.guard_direction;
        let mut visited: Grid<Option<Direction>> =
            Grid::new(self.map.width() as usize, self.map.height() as usize);

        loop {
            let mut next_position = guard_position + &guard_direction;
            let mut next_element = self.map.get(next_position);

            while next_position == extra_obstacle || next_element == Some(&MapElement::Obstacle) {
                guard_direction = guard_direction.clockwise_4();
                next_position = guard_position + &guard_direction;
                next_element = self.map.get(next_position);
            }

            // Guard walked off the map
            if next_element.is_none() {
                return false;
            }

            // Guard walked into a loop
            if visited[next_position] == Some(guard_direction) {
                return true;
            };

            // walk to the next step
            visited[next_position] = Some(guard_direction);
            guard_position = next_position;
        }
    }
}

pub fn solve(input: &PuzzleInput) -> String {
    let state = MapState::new(&input.map);

    input
        .map
        .iter_coordinates()
        .filter(|&coordinate| input.map[coordinate] == MapElement::Empty)
        .filter(|&coordinate| state.does_loop(coordinate))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "6")]
    #[ignore]
    #[case::final_input( include_str!("../input.txt"), "UNSOLVED")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
