use crate::puzzle_input::PuzzleInput;

use memoize::memoize;

#[memoize]
fn count_expansion(input: u128, expansions: usize) -> u128 {
    if expansions == 0 {
        return 1;
    }

    if input == 0 {
        return count_expansion(1, expansions - 1);
    }

    let digits = input.ilog10() + 1;
    if digits % 2 == 0 {
        let splitting_power = 10u128.pow(digits / 2);
        let remainder = input % splitting_power;
        let quotient = input / splitting_power;
        count_expansion(remainder, expansions - 1) + count_expansion(quotient, expansions - 1)
    } else {
        count_expansion(input * 2024, expansions - 1)
    }
}

pub fn solve(input: &PuzzleInput) -> String {
    input
        .numbers
        .iter()
        .map(|number| count_expansion(*number, 25))
        .sum::<u128>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "55312")]
    #[case::final_input( include_str!("../input.txt"), "202019")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
