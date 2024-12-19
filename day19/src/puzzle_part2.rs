use crate::puzzle::PuzzleInput;
use pathfinding::prelude::count_paths;

pub trait Part2 {
    fn part2(&self) -> String;
}

impl Part2 for PuzzleInput<'_> {
    fn part2(&self) -> String {
        self.target_patterns
            .iter()
            .map(|&target_pattern| {
                count_paths(
                    target_pattern,
                    |pattern| {
                        self.towel_patterns
                            .iter()
                            .filter_map(|&towel_pattern| pattern.strip_prefix(towel_pattern))
                            .collect::<Vec<_>>()
                    },
                    |pattern| pattern.is_empty(),
                )
            })
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "16")]
    #[case::final_input( include_str!("../input.txt"), "UNSOLVED")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part2(), expected);
    }
}
