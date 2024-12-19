use std::cmp::min;

use nom::{
    self,
    bytes::complete::{is_a, tag},
    character::complete::{multispace0, multispace1},
    combinator::{map, verify},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

use crate::puzzle::PuzzleInput;

fn parse_puzzle<'a>(input: &'a str) -> IResult<&'a str, PuzzleInput<'a>> {
    // Main parser for the puzzle
    let puzzle_parser = map(
        separated_pair(
            separated_list1(tag(", "), is_a("rwbug")),
            verify(multispace1, |v: &str| v.len() > 1),
            separated_list1(multispace1, is_a("rwbug")),
        ),
        |(towel_patterns, target_patterns)| PuzzleInput::new(towel_patterns, target_patterns),
    );

    // strip whitespace around the input (copy-pasting can me inprecise with respect to whitespace)
    let mut parser = delimited(multispace0, puzzle_parser, multispace0);

    parser(input)
}

impl<'a> TryFrom<&'a str> for PuzzleInput<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match parse_puzzle(s).finish() {
            Ok((_remaining, puzzle_input)) => {
                puzzle_input
                    .validate_assumptions()
                    .map_err(|e| e.to_string())?;
                Ok(puzzle_input)
            }
            Err(Error { input, code }) => Err(format!(
                "Parse Error {:?} just before {}",
                code,
                &input[..min(30, input.len())]
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example_input1(include_str!("../example_input.txt"))]
    #[case::final_input(include_str!("../input.txt"))]
    /// Verifies that the test input is valid.
    fn test_puzzle_input_from_example_input(#[case] input: &str) {
        let puzzle = PuzzleInput::try_from(input).unwrap();
        puzzle.validate_assumptions().unwrap()
    }

    #[rstest]
    #[case::example_input("Not valid input")]
    #[case::final_input("")]
    /// Verifies that invalid input is rejected.
    fn test_puzzle_input_from_str_bad(#[case] input: &str) {
        let input = PuzzleInput::try_from(input);
        assert!(input.is_err());
    }
}
