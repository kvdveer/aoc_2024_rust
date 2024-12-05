use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, u8},
    combinator::map,
    error::Error,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    pub rules: Vec<(u8, u8)>,
    pub sequences: Vec<Vec<u8>>,
}

fn parse_puzzle(input: &str) -> IResult<&str, PuzzleInput> {
    // Main parser for the puzzle

    let rule_parser = separated_pair(u8, tag("|"), u8);
    let rules_parser = separated_list1(line_ending, rule_parser);
    let sequence_parser = separated_list1(tag(","), u8);
    let sequences_parser = separated_list1(line_ending, sequence_parser);

    let puzzle_parser = map(
        separated_pair(rules_parser, many1(line_ending), sequences_parser),
        |(rules, sequences)| PuzzleInput { rules, sequences },
    );

    // strip whitespace around the input (copy-pasting can be inprecise with respect to whitespace)
    let mut parser = delimited(multispace0, puzzle_parser, multispace0);

    parser(input)
}

impl TryFrom<&str> for PuzzleInput {
    type Error = Error<String>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match parse_puzzle(s).finish() {
            Ok((_remaining, puzzle_input)) => Ok(puzzle_input),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[rstest]
    #[case::example_input(EXAMPLE_INPUT)]
    #[case::final_input(INPUT)]
    /// Verifies that the test input is valid.
    fn test_puzzle_input_from_example_input(#[case] input: &str) {
        let input = PuzzleInput::try_from(input).unwrap();
        assert!(!input.sequences.is_empty());
        assert!(!input.rules.is_empty());
    }

    #[rstest]
    #[case::example_input("Not valid input")]
    #[case::final_input("")]
    /// Verifes that invalid input is rejected.
    fn test_puzzle_input_from_str_bad(#[case] input: &str) {
        let input = PuzzleInput::try_from(input);
        assert!(input.is_err());
    }
}
