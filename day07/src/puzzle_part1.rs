use crate::puzzle_input::{Equation, PuzzleInput};

fn partial_solve(operands: &[u64], outcome: u64) -> bool {
    assert!(!operands.is_empty());
    if operands.len() == 1 {
        return operands[0] == outcome;
    }

    let final_operand = operands[operands.len() - 1];
    let remaining_operands = &operands[0..operands.len() - 1];

    // Try to solve with addition operand
    if outcome >= final_operand && partial_solve(remaining_operands, outcome - final_operand) {
        return true;
    }

    if outcome % final_operand == 0 && partial_solve(remaining_operands, outcome / final_operand) {
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
    #[case::example_input(include_str!("../example_input.txt"), "3749")]
    #[case::final_input( include_str!("../input.txt"), "3245122495150")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
