use crate::puzzle::PuzzleInput;
use pathfinding::prelude::bfs;

pub trait Part1 {
    fn part1(&self) -> String;
}

impl Part1 for PuzzleInput<'_> {
    fn part1(&self) -> String {
        self.target_patterns
            .iter()
            .filter(|&&target_pattern| {
                bfs(
                    &target_pattern,
                    |pattern| {
                        self.towel_patterns
                            .iter()
                            .filter_map(|&towel_pattern| pattern.strip_prefix(towel_pattern))
                            .collect::<Vec<_>>()
                    },
                    |pattern| pattern.is_empty(),
                )
                .is_some()
            })
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "6")]
    #[case::final_input( include_str!("../input.txt"), "317")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part1(), expected);
    }
}
