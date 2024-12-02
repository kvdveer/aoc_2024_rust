use nom::{
    self,
    character::complete::{line_ending, multispace0, not_line_ending, space1, i8},
    combinator::{map, peek},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, tuple},
    Finish, IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput<'puzzle> {
    // The raw, unparsed lines of the input. Added here for convenience, in case parsing loses some information.
    pub raw_lines: Vec<&'puzzle str>,
    
    pub reports: Vec<Vec<i8>>,
}

fn parse_puzzle(input: &str) -> IResult<&str, PuzzleInput> {
    let puzzle_input = separated_list1(line_ending, separated_list1(space1, i8));

    // strip whitespace around the input (copy-pasting can me inprecise with respect to whitespace)
    let mut parser = delimited(
        multispace0,
        map(
            // Use peek to parse two ways (just the lines, then the actual parsed input)
            tuple((
                peek(separated_list1(line_ending, not_line_ending)),
                puzzle_input,
            )),
            |(raw_lines, reports)| PuzzleInput { raw_lines, reports },
        ),
        multispace0,
    );

    parser(input)
}

impl<'puzzle> TryFrom<&'puzzle str> for PuzzleInput<'puzzle> {
    type Error = Error<&'puzzle str>;

    fn try_from(s: &'puzzle str) -> Result<Self, Self::Error> {
        match parse_puzzle(s).finish() {
            Ok((_remaining, puzzle_input)) => Ok(puzzle_input),
            Err(Error { input, code }) => Err(Error { input, code }),
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
        assert!(!input.raw_lines.is_empty());
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
