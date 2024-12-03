use crate::puzzle_input::PuzzleInput;
use regex::Regex;

pub fn solve(input: &PuzzleInput) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex is valid");

    input
        .raw_lines
        .iter()
        .flat_map(|line| {
            re.captures_iter(line)
                .map(|c| c[1].parse::<u64>().unwrap() * c[2].parse::<u64>().unwrap())
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input.txt"), "161")]
    #[case::final_input( include_str!("../input.txt"), "192767529")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
