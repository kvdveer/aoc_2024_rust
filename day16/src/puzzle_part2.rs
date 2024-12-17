use std::collections::HashSet;

use aoc_grid::{Coordinate, Direction, Grid};
use pathfinding::prelude::astar_bag_collect;

use crate::puzzle::{MapState, PuzzleInput};

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

pub trait Part2 {
    fn part2(&self) -> String;
}

impl Part2 for PuzzleInput {
    fn part2(&self) -> String {
        let (paths, _) = astar_bag_collect(
            &RacerState {
                pos: self.start,
                direction: Direction::Right,
            },
            |p| p.successors(&self.map),
            |p| p.pos.manhattan_distance(self.finish) as u32,
            |p| p.pos == self.finish,
        )
        .expect("There's always a path");

        HashSet::<Coordinate>::from_iter(paths.iter().flat_map(|path| path.iter().map(|p| p.pos)))
            .len()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input1.txt"), "45")]
    #[case::example_input(include_str!("../example_input2.txt"), "64")]
    #[case::final_input( include_str!("../input.txt"), "541")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part2(), expected);
    }
}
