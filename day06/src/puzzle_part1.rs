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
    visited: Grid<Visited>,
    guard_position: Coordinate,
    guard_direction: Direction,
}

impl MapState {
    fn new(base_map: &Grid<MapElement>) -> Self {
        let mut result = Self {
            // Remove the guard from the map
            map: base_map.map(|&c| match c {
                MapElement::Guard => MapElement::Empty,
                _ => c,
            }),
            // Mark no position as visited yet
            visited: Grid::new(base_map.width(), base_map.height()),

            // Look up the guard position
            guard_position: base_map
                .iter_coordinates()
                .find(|&c| base_map[c] == MapElement::Guard)
                .expect("map should have a guard position"),

            // Guard always starts facing up
            guard_direction: Direction::Up,
        };

        result.visited[result.guard_position] = Visited::Visited;
        result
    }

    fn step(&mut self) -> bool {
        let mut next_position = self.guard_position + &self.guard_direction;
        let mut next_element = self.map.get(next_position);

        while next_element == Some(&MapElement::Obstacle) {
            self.guard_direction = self.guard_direction.clockwise_4();
            next_position = self.guard_position + &self.guard_direction;
            next_element = self.map.get(next_position);
        }

        if next_element.is_some() {
            self.guard_position = next_position;
            self.visited[self.guard_position] = Visited::Visited;
            true
        } else {
            false
        }
    }
}

pub fn solve(input: &PuzzleInput) -> String {
    let mut state = MapState::new(&input.map);

    while state.step() {}

    state
        .visited
        .iter()
        .filter(|&&v| v == Visited::Visited)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    // #[ignore]
    #[case::example_input(include_str!("../example_input.txt"), "41")]
    #[case::final_input( include_str!("../input.txt"), "4758")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
