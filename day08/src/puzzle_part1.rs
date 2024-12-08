use std::collections::HashSet;

use aoc_grid::Grid;

use crate::puzzle_input::{MapElement, PuzzleInput};

pub fn solve(input: &PuzzleInput) -> String {
    let mut antinodes = Grid::<bool>::new(input.cells.width(), input.cells.height());

    // Find the distinct frequencies
    let frequencies = input
        .cells
        .iter()
        .fold(HashSet::<char>::new(), |mut acc, &cell| {
            if let MapElement::Antenna(c) = cell {
                acc.insert(c);
            };
            acc
        });

    // For each frequency, find the broadcasting antennas
    frequencies.iter().for_each(|f| {
        let antennas = input
            .cells
            .iter_pairs()
            .filter_map(|(coord, cell)| {
                if let MapElement::Antenna(c) = cell {
                    if c == f {
                        Some(coord)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // For each pair of antennas, compute the 'antinodes'.
        antennas.iter().enumerate().for_each(|(i, &a)| {
            antennas[i + 1..].iter().for_each(|&b| {
                let delta = a - b;
                let antinode1 = a + delta;
                let antinode2 = b - delta;

                if antinodes.coordinate_valid(antinode1) {
                    antinodes[antinode1] = true;
                }
                if antinodes.coordinate_valid(antinode2) {
                    antinodes[antinode2] = true;
                }
            });
        });
    });

    // Count the number of antinodes
    let count = antinodes.iter().filter(|&&b| b).count();
    count.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "14")]
    #[case::final_input( include_str!("../input.txt"), "396")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
