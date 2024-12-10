use aoc_grid::{Coordinate, Direction, Grid};

use crate::puzzle_input::PuzzleInput;

fn follow_path_uphill(start: Coordinate, elevations: &Grid<u8>) -> Vec<Coordinate> {
    let start_elevation = elevations[start];

    if start_elevation == 9 {
        // We're at the top of the mountain.
        return vec![start];
    }

    let result_iter = Direction::CARDINAL_4
        .iter()
        .map(|dir| start + dir)
        .filter(|next| elevations.get(next) == Some(&(start_elevation + 1)))
        .map(|next| follow_path_uphill(next, elevations).to_vec());

    result_iter.fold(vec![], |mut acc, summits| {
        acc.extend(summits.iter());
        acc
    })
}

pub fn solve(input: &PuzzleInput) -> String {
    input
        .elevations
        .iter_pairs()
        .filter(|(_coord, elevation)| **elevation == 0)
        .map(|(coord, _elevation)| {
            let mut summits = follow_path_uphill(coord, &input.elevations);
            summits.sort_by_key(|c| c.0 + 100 * c.1);
            summits.dedup();
            summits.len()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "36")]
    #[case::final_input( include_str!("../input.txt"), "698")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
