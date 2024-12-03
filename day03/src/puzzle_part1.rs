use crate::puzzle_input::PuzzleInput;
use regex::bytes::Regex;

fn bytes_to_u64(bytes: &[u8]) -> u64 {
    let mut value=0;
    for b in bytes {
        value = value*10 + (b - b'0') as u64;
    }

    value
}

pub fn solve(input: &PuzzleInput) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex is valid");

    input
        .raw_lines
        .iter()
        .flat_map(|line| {
            re.captures_iter(line.as_bytes()).map(|c| {
                let a = bytes_to_u64(&c[1]);
                let b = bytes_to_u64(&c[2]);
                a * b
            })
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
