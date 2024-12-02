use crate::puzzle_input::PuzzleInput;

pub fn solve(_input: &PuzzleInput) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[ignore]
    #[case::example_input(include_str!("../example_input.txt"), "UNSOLVED")]
    #[ignore]
    #[case::final_input( include_str!("../input.txt"), "UNSOLVED")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}