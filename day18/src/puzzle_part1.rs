use crate::puzzle::PuzzleInput;
use aoc_grid::{Coordinate, Direction, Grid};
use pathfinding::prelude::astar;

pub trait Part1 {
    fn part1(&self) -> String;
}

impl Part1 for PuzzleInput {
    fn part1(&self) -> String {
        // For part1 we only use some of the fallen blocks
        let limit = if self.grid_width == 7 { 12 } else { 1024 };

        // Build a map of the time each byte will fall
        let mut fallen_blocks = Grid::<bool>::new(self.grid_width, self.grid_height);
        self.falling_bytes.iter().take(limit).for_each(|coord| {
            fallen_blocks[coord] = true;
        });

        // Find coordinates of the start and destination
        let start = Coordinate(0, 0);
        let destination = Coordinate(
            (self.grid_width - 1) as isize,
            (self.grid_height - 1) as isize,
        );

        if let Some((_path, cost)) = astar(
            &start,
            |&coord| {
                Direction::CARDINAL_4
                    .iter()
                    .map(move |dir| coord + dir)
                    .filter(|next| fallen_blocks.get(*next) == Some(&false))
                    .map(|coord| (coord, 1))
            },
            |&coord| destination.manhattan_distance(coord),
            |&coord| coord == destination,
        ) {
            cost.to_string()
        } else {
            "UNSOLVABLE".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "22")]
    #[case::final_input( include_str!("../input.txt"), "268")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part1(), expected);
    }
}
