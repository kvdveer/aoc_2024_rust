use nom::{
    self,
    character::complete::{i32, multispace0, newline, space1},
    combinator::map,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct PuzzleInput {
    pub pairs: Vec<(i32, i32)>,
}

fn parse_puzzle(input: &str) -> IResult<&str, PuzzleInput> {
    let puzzle_input = separated_list1(newline, separated_pair(i32, space1, i32));

    // strip whitespace around the input (copy-pasting can me inprecise with respect to whitespace)
    let mut parser = delimited(
        multispace0,
        map(
            // Use peek to parse two ways (just the lines, then the actual parsed input)
            puzzle_input,
            |pairs| PuzzleInput { pairs },
        ),
        multispace0,
    );

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
        assert!(!input.pairs.is_empty());
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
