use crate::puzzle_input::PuzzleInput;
use std::collections::BTreeSet;

fn sequence_is_valid(rules: &BTreeSet<(u8, u8)>, sequence: &[u8]) -> bool {
    sequence.iter().enumerate().all(|(i, &left)| {
        sequence[i + 1..]
            .iter()
            .all(|&right| !rules.contains(&(right, left)))
    })
}

fn order_sequence(rules: &[(u8, u8)], sequence: &[u8]) -> Vec<u8> {
    let mut tokens = sequence.to_vec();

    // Filter out the relevant rules
    let mut relevant_rules = rules
        .iter()
        .filter(|(l, r)| sequence.contains(l) && sequence.contains(r))
        .collect::<Vec<_>>();

    let mut result = Vec::with_capacity(sequence.len());

    while !relevant_rules.is_empty() {
        // Find a token that is _not_ placed after any other token.
        let (idx, &token) = tokens
            .iter()
            .enumerate()
            .find(|(_, token)| !relevant_rules.iter().any(|(_, r)| r == *token))
            .expect("Rules should be consistent");

        result.push(token);
        tokens.swap_remove(idx);
        relevant_rules.retain(|(l, r)| *l != token && *r != token);
    }

    result.extend(0u8..(sequence.len() - result.len()) as u8);
    result
}

pub fn solve(input: &PuzzleInput) -> String {
    let rules = BTreeSet::from_iter(input.rules.iter().copied());

    input
        .sequences
        .iter()
        .filter(|seq| !sequence_is_valid(&rules, seq))
        .map(|seq| {
            let ordered = order_sequence(&input.rules, seq);
            ordered[ordered.len() / 2] as u16
        })
        .sum::<u16>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    // #[ignore]
    #[case::example_input(include_str!("../example_input.txt"), "123")]
    #[case::final_input( include_str!("../input.txt"), "4922")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
