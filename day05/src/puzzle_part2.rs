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
    let mut tokens = sequence.iter().copied().collect::<BTreeSet<_>>();

    // Filter out the relevant rules
    let mut relevant_rules = rules
        .iter()
        .filter(|(l, r)| sequence.contains(l) && sequence.contains(r))
        .copied()
        .collect::<Vec<_>>();

    let mut result = Vec::new();

    while !relevant_rules.is_empty() {
        // Find a token that is _not_ placed after any other token.
        let token = tokens
            .iter()
            .copied()
            .find(|token| !relevant_rules.iter().any(|(_, r)| r == token))
            .expect("Rules should be consistent");

        result.push(token);
        tokens.remove(&token);
        relevant_rules.retain(|(l, r)| *l != token && *r != token);
    }

    result.extend(tokens.iter());
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

    #[rstest]
    #[case::example_sequence(include_str!("../example_input.txt"), vec![75,97,47,61,53], vec![97,75,47,61,53])]
    fn test_ordered(
        #[case] puzzle_input: &str,
        #[case] sequence: Vec<u8>,
        #[case] expected: Vec<u8>,
    ) {
        let input = PuzzleInput::try_from(puzzle_input).unwrap();
        let sorted = order_sequence(&input.rules, &sequence);
        assert_eq!(sorted, expected);
    }
}
