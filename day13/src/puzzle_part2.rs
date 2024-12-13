use crate::puzzle_input::{ClawMachine, PuzzleInput, Vector};

use crate::solver::Solver;

pub fn solve(input: &PuzzleInput) -> String {
    input
        .claw_machines
        .iter()
        .map(|m| ClawMachine {
            a: m.a.clone(),
            b: m.b.clone(),
            target: Vector {
                x: m.target.x + 10000000000000,
                y: m.target.y + 10000000000000,
            },
        })
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
    #[case::example_input(include_str!("../example_input.txt"), "480")]
    #[ignore]
    #[case::final_input( include_str!("../input.txt"), "UNSOLVED")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
