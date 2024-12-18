use aoc_grid::{Coordinate, Direction};

use crate::puzzle::PuzzleInput;
use pathfinding::prelude::astar;

pub trait Part2 {
    fn part2(&self) -> String;
}

fn path_exists(falling_blocks: &[Coordinate], width: usize, height: usize) -> bool {
    let start = Coordinate(0, 0);
    let finish = Coordinate(width as isize - 1, height as isize - 1);
    let result = astar(
        &start,
        |&coord|// vec![],
        {
            Direction::CARDINAL_4
                .iter()
                .map(move |dir| coord + dir)
                .filter(|next| {
                    next.0 >= 0
                        && next.0 < width as isize
                        && next.1 >= 0
                        && next.1 < height as isize
                        && !falling_blocks.contains(next)
                }).map(|coord| (coord, 1))
        },
        |&coord| finish.manhattan_distance(coord),
        |&coord| coord == finish,
    )
    .is_some();
    result
}

impl Part2 for PuzzleInput {
    fn part2(&self) -> String {
        // Perform a binary search to find the first coordinate where no path exists
        let mut bounds = 0..self.falling_bytes.len();
        while bounds.len() > 1 {
            let mid = bounds.start + bounds.len() / 2;
            if path_exists(
                &self.falling_bytes[..mid],
                self.grid_width,
                self.grid_height,
            ) {
                bounds = mid..bounds.end;
            } else {
                bounds = bounds.start..mid;
            }
        }
        let fatal_coordinate = self.falling_bytes[bounds.start];
        format!("{},{}", fatal_coordinate.0, fatal_coordinate.1)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "6,1")]
    #[ignore]
    #[case::final_input( include_str!("../input.txt"), "UNSOLVED")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part2(), expected);
    }
}
