use aoc_grid::Direction;

use crate::puzzle_input::PuzzleInput;

pub fn solve(input: &PuzzleInput) -> String {
    let needle = "XMAS".chars().collect::<Vec<_>>();

    input
        .letters
        // Check each grid cell
        .iter_coordinates()
        .filter(|coord| input.letters[coord] == needle[0])
        .map(|coord| {
            // Check each direction
            Direction::CARDINAL_8
                .iter()
                .filter(|&dir| {
                    // for each character in the needle, check if the character at the current
                    needle
                        .iter()
                        .enumerate()
                        // reverse to make sure we're checking the furthest character first, it gives an early-out for out-of-bounds
                        .rev()
                        .all(|(idx, c)| input.letters.get((dir * idx) + coord) == Some(c))
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "18")]
    #[case::final_input( include_str!("../input.txt"), "2397")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
