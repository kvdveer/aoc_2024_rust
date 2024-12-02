use crate::puzzle_input::PuzzleInput;

pub fn solve(input: &PuzzleInput) -> String {
    input
        .reports
        .iter()
        .filter(|report| {
            let mut diffs = report.windows(2).map(|window| window[0] - window[1]);

            match diffs.next() {
                Some(1) | Some(2) | Some(3) => diffs.all(|diff| (1..=3).contains(&diff)),
                Some(-1) | Some(-2) | Some(-3) => diffs.all(|diff| (-3..=-1).contains(&diff)),
                _ => false,
            }
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");
    const PUZZLE_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_solve() {
        let input = PuzzleInput::try_from(EXAMPLE_INPUT).unwrap();
        assert_eq!(solve(&input), "2");
    }

    #[test]
    fn test_correct_answer() {
        let input = PuzzleInput::try_from(PUZZLE_INPUT).unwrap();
        assert_eq!(solve(&input), "299");
    }
}
