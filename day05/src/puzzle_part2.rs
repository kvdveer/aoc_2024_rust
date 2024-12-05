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
    // Filter out the relevant rules
    let mut relevant_rules = rules
        .iter()
        .filter(|(l, r)| sequence.contains(l) && sequence.contains(r))
        .copied()
        .collect::<BTreeSet<_>>();

    // Exend the rules with implied rules (e.g. A|B and B|C implies A|C)
    loop {
        let rules_at_start = relevant_rules.len();
        let implied_rules = relevant_rules
            .iter()
            .flat_map(|(left, middle)| {
                relevant_rules.iter().filter_map(|(middle2, right)| {
                    if *middle2 == *middle {
                        Some((*left, *right))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        relevant_rules.extend(implied_rules.iter());

        // When there's no more rules added, all implied rules are found
        if relevant_rules.len() == rules_at_start {
            break;
        }
        dbg!(relevant_rules.len());
    }

    let mut result = sequence.to_vec();
    result.sort_by_cached_key(|token| relevant_rules.iter().filter(|(_, r)| r == token).count());
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
    #[ignore]
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
