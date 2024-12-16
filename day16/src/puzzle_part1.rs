use crate::puzzle::{MapState, PuzzleInput};
use aoc_grid::{Coordinate, Direction, Grid};
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct RacerState {
    pos: Coordinate,
    direction: Direction,
}

impl RacerState {
    fn successors(&self, map: &Grid<MapState>) -> Vec<(RacerState, u32)> {
        let mut result = Vec::with_capacity(4);

        // Add rotations with a fee of 1000000
        result.extend((1..=3).map(|r| {
            (
                RacerState {
                    pos: self.pos,
                    direction: self.direction.rotate_clockwise_4(r),
                },
                1_000,
            )
        }));

        // If the next tile add it with a fee of 1000, or 1001 if it's already visited (to prevent revisits)

        if map.get(self.pos + &self.direction) == Some(&MapState::Empty) {
            result.push((
                RacerState {
                    pos: self.pos + &self.direction,
                    direction: self.direction,
                },
                1,
            ));
        }

        result
    }
}

pub trait Part1 {
    fn part1(&self) -> String;
}

impl Part1 for PuzzleInput {
    fn part1(&self) -> String {
        let result = dijkstra(
            &RacerState {
                pos: self.start,
                direction: Direction::Right,
            },
            |p| p.successors(&self.map),
            |p| p.pos == self.finish,
        );

        match result {
            Some((_, cost)) => cost.to_string(),
            None => "No path found".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input1.txt"), "7036")]
    #[case::example_input(include_str!("../example_input2.txt"), "11048")]
    #[case::final_input( include_str!("../input.txt"), "135512")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part1(), expected);
    }
}
