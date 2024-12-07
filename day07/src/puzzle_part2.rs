use crate::puzzle_input::{Equation, PuzzleInput};

fn partial_solve(operands: &[u64], outcome: u64) -> bool {
    assert!(!operands.is_empty());
    if operands.len() == 1 {
        return operands[0] == outcome;
    }

    let final_operand = operands[operands.len() - 1];
    let remaining_operands = &operands[0..operands.len() - 1];

    if outcome < final_operand {
        // All operators increase outcome, so we can never reach the outcome with the remaining operands
        return false;
    }

    // Try to solve with addition operand
    if partial_solve(remaining_operands, outcome - final_operand) {
        return true;
    }

    if outcome % final_operand == 0 && partial_solve(remaining_operands, outcome / final_operand) {
        return true;
    }

    let powers: [u64; 20] = [
        1,
        10,
        100,
        1000,
        10000,
        100000,
        1000000,
        10000000,
        100000000,
        1000000000,
        10000000000,
        100000000000,
        1000000000000,
        10000000000000,
        100000000000000,
        1000000000000000,
        10000000000000000,
        100000000000000000,
        1000000000000000000,
        10000000000000000000,
    ];

    let idx = powers.partition_point(|&c| c <= final_operand);
    let power = powers[idx];

    // Find the power of 10 that is the largest number that is less than the outcome
    // let power = powers.iter().find(|&p| *p > final_operand).expect("Power table is large enough");
    // assert!(powers[x] == *power);

    if outcome % power == final_operand && partial_solve(remaining_operands, outcome / power) {
        return true;
    }

    false
}

fn solve_equation(equation: &Equation) -> bool {
    partial_solve(&equation.operands, equation.outcome)
}

pub fn solve(input: &PuzzleInput) -> String {
    input
        .equations
        .iter()
        .filter(|equation| solve_equation(equation))
        .map(|eq| eq.outcome)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "11387")]
    #[case::final_input( include_str!("../input.txt"), "105517128211543")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }

    #[rstest]
    #[case::example_input1(&[15, 6], 156, true)]
    fn test_partial_solve(#[case] operands: &[u64], #[case] outcome: u64, #[case] expected: bool) {
        assert_eq!(partial_solve(operands, outcome), expected);
    }
}
