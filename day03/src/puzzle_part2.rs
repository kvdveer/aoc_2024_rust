
use regex::bytes::Regex;

use crate::puzzle_input::PuzzleInput;

fn bytes_to_u64(bytes: &[u8]) -> u64 {
    let mut value=0;
    for b in bytes {
        value = value*10 + (b - b'0') as u64;
    }

    value
}
pub fn solve(input: &PuzzleInput) -> String {
    let re = Regex::new(r"(do|don't)\(\)|mul\((\d+),(\d+)\)").expect("Regex is valid");

    let mut sum: u64 = 0;
    let mut enabled = true;
    for capture in input
        .raw_lines
        .iter()
        .flat_map(|line| re.captures_iter(line.as_bytes()))
    {
        match capture.get(1).map(|m| m.as_bytes()) {
            Some(b"do") => enabled = true,
            Some(b"don't") => enabled = false,
            None => {
                if enabled {
                    let a = bytes_to_u64(&capture[2]);
                    let b = bytes_to_u64(&capture[3]);
                    sum += a * b
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
