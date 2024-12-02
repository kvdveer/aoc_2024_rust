use crate::puzzle_input::PuzzleInput;

fn is_report_valid(report: &[i8]) -> bool {
    let mut diffs = report.windows(2).map(|window| window[0] - window[1]);

    match diffs.next() {
        Some(1) | Some(2) | Some(3) => diffs.all(|diff| (1..=3).contains(&diff)),
        Some(-1) | Some(-2) | Some(-3) => diffs.all(|diff| (-3..=-1).contains(&diff)),
        _ => false,
    }
}

pub fn solve(input: &PuzzleInput) -> String {
    input
        .reports
        .iter()
        .filter(|report| {
            if is_report_valid(report) {
                return true;
            }

            // Try removing one element at a time to see if the report is valid
            for (i, _) in report.iter().enumerate() {
                let report = report
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, x)| *x)
                    .collect::<Vec<i8>>();

                if is_report_valid(&report) {
                    return true;
                }
            }

            false
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "4")]
    #[case::final_input( include_str!("../input.txt"), "364")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
