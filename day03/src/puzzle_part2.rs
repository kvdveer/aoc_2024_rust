use regex::Regex;

use crate::puzzle_input::PuzzleInput;

pub fn solve(input: &PuzzleInput) -> String {
    let re = Regex::new(r"(do|don't)\(\)|mul\((\d+),(\d+)\)").expect("Regex is valid");

    let mut sum: u64 = 0;
    let mut enabled = true;
    for capture in input
        .raw_lines
        .iter()
        .flat_map(|line| re.captures_iter(line))
    {
        match capture.get(1).map(|m| m.as_str()) {
            Some("do") => enabled = true,
            Some("don't") => enabled = false,
            None => {
                if enabled {
                    sum += capture[2].parse::<u64>().unwrap() * capture[3].parse::<u64>().unwrap();
                }
            }
            _ => unreachable!(),
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input(include_str!("../example_input2.txt"), "48")]
    #[case::final_input( include_str!("../input.txt"), "104083373")]
    fn test_solve(#[case] input: &str, #[case] expected: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert_eq!(solve(&input), expected);
    }
}
