use crate::puzzle_input::PuzzleInput;
use std::collections::BTreeSet;

fn sequence_is_valid(rules: &BTreeSet<(u8, u8)>, sequence: &[u8]) -> bool {
    sequence.iter().enumerate().all(|(i, &left)| {
        sequence[i + 1..]
            .iter()
            // Try to find a contradicting rule for each pair of elements
            .all(|&right| !rules.contains(&(right, left)))
    })
}

pub fn solve(input: &PuzzleInput) -> String {
    let rules = BTreeSet::from_iter(input.rules.iter().copied());

    input
        .sequences
        .iter()
        .filter(|sequence| sequence_is_valid(&rules, sequence))
        // Find the middle element of each sequence and sum them
        .map(|seq| seq[seq.len() / 2] as u16)
        .sum::<u16>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "143")]
    #[case::final_input( include_str!("../input.txt"), "3608")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
