use crate::puzzle_input::PuzzleInput;

use crate::solver::Solver;

pub fn solve(input: &PuzzleInput) -> String {
    input
        .claw_machines
        .iter()
        .flat_map(|m| m.solve())
        .map(|(a, b)| 3 * a + b)
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "875318608908")]
    #[case::final_input( include_str!("../input.txt"), "72587986598368")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
