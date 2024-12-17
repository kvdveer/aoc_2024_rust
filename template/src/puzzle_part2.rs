use crate::puzzle::PuzzleInput;

pub trait Part2 {
    fn part2(&self) -> String;
}

impl Part2 for PuzzleInput {
    fn part2(&self) -> String {
        "UNSOLVED".to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "UNSOLVED")]
    #[ignore]
    #[case::final_input( include_str!("../input.txt"), "UNSOLVED")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(input.part2(), expected);
    }
}
