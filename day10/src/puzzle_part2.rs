use aoc_grid::{Coordinate, Direction, Grid};

use crate::puzzle_input::PuzzleInput;

fn follow_path_uphill(start: Coordinate, elevations: &Grid<u8>) -> usize {
    let start_elevation = elevations[start];

    if start_elevation == 9 {
        // We're at the top of the mountain.
        return 1;
    }

    Direction::CARDINAL_4
        .iter()
        .map(|dir| start + dir)
        .filter(|next| elevations.get(next) == Some(&(start_elevation + 1)))
        .map(|next| follow_path_uphill(next, elevations))
        .sum::<usize>()
}

pub fn solve(input: &PuzzleInput) -> String {
    input
        .elevations
        .iter_pairs()
        .filter(|(_coord, elevation)| **elevation == 0)
        .map(|(coord, _elevation)| follow_path_uphill(coord, &input.elevations))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "81")]
    #[case::final_input( include_str!("../input.txt"), "1436")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
