use std::{cmp::max, collections::HashSet};

use aoc_grid::{grid, Grid};

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
                let delta_magnitude = max(delta.0.abs(), delta.1.abs());
                let grid_magnitude = max(antinodes.width(), antinodes.height()) as isize;

                let max_repetitions = grid_magnitude / delta_magnitude + 1;

                (-max_repetitions..=max_repetitions).for_each(|n| {
                    let antinode = a + delta * n;
                    if antinodes.coordinate_valid(antinode) {
                        antinodes[antinode] = true;
                    }
                });
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
    #[case::example_input(include_str!("../example_input.txt"), "34")]
    #[case::final_input( include_str!("../input.txt"), "1200")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
